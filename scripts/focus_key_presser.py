#!/usr/bin/env python3
"""
Focus Key Presser (X11/XWayland, KDE Plasma on Fedora)
- Press one or more keys at independent intervals while a specific window/app is focused.
- Matching by title, WM_CLASS, .exe name (Proton/Wine), or PID.
- Includes a discover mode to show you exactly what to enter.

Dependencies (Fedora):
  sudo dnf install python3-Xlib

Wayland vs X11:
- This uses X11 XTest to inject keys. Under Wayland, generic key injection is restricted.
- It works if the target window is X11/XWayland (DISPLAY is set). Many Proton games are XWayland.
- For Wayland-native windows, ask for a portal-based (xdg-desktop-portal RemoteDesktop) variant.

SELinux:
- Works under default enforcing SELinux with no policy changes.
- Connects to your X server and reads /proc of your own processes only.
"""

import argparse
import os
import sys
import time
import logging
import traceback

# Lazy import handling for clearer error messages
try:
    from Xlib import X, XK, display
    from Xlib.ext import xtest
    from Xlib.error import BadWindow, XError
except Exception:
    pass

LOG = logging.getLogger("focus-key-presser")


def have_x11() -> bool:
    return bool(os.environ.get("DISPLAY"))


def safe_read(path: str, mode="rb"):
    try:
        with open(path, mode) as f:
            return f.read()
    except Exception:
        return None


def get_ppid(pid: int) -> int | None:
    data = safe_read(f"/proc/{pid}/status", "r")
    if not data:
        return None
    for line in data.splitlines():
        if line.startswith("PPid:"):
            try:
                return int(line.split()[1])
            except Exception:
                return None
    return None


def get_comm(pid: int) -> str | None:
    data = safe_read(f"/proc/{pid}/comm", "r")
    if not data:
        return None
    return data.strip()


def get_cmdline_tokens(pid: int) -> list[str]:
    b = safe_read(f"/proc/{pid}/cmdline", "rb")
    if not b:
        return []
    parts = b.split(b"\x00")
    out = []
    for p in parts:
        if not p:
            continue
        try:
            out.append(p.decode("utf-8", errors="ignore"))
        except Exception:
            out.append(p.decode(errors="ignore"))
    return out


def exe_candidates_from_tokens(tokens: list[str]) -> list[str]:
    """
    Extract possible .exe basenames from Wine/Proton cmdline tokens.
    Handles tokens like:
      'Z:\\...\\Shape of Dreams.exe'
      'C:\\Program Files\\Game\\Game.exe'
      '/home/user/.steam/.../drive_c/.../Game.exe'
    """
    import os
    cands = []
    for t in tokens:
        if not t:
            continue
        tt = t.strip().strip('"').strip("'")
        tt_norm = tt.replace("\\", "/")
        base = os.path.basename(tt_norm)
        if base.lower().endswith(".exe"):
            cands.append(base)
    seen = set()
    out = []
    for c in cands:
        lc = c.lower()
        if lc in seen:
            continue
        seen.add(lc)
        out.append(c)
    return out


class X11Inspector:
    def __init__(self, display_name: str | None = None):
        self.display_name = display_name or os.environ.get("DISPLAY")
        self.d = display.Display(self.display_name)
        self.root = self.d.screen().root

        # Atoms
        self.NET_ACTIVE_WINDOW = self.d.intern_atom("_NET_ACTIVE_WINDOW")
        self.NET_WM_NAME = self.d.intern_atom("_NET_WM_NAME")
        self.UTF8_STRING = self.d.intern_atom("UTF8_STRING")
        self.WM_NAME = self.d.intern_atom("WM_NAME")
        self.WM_CLASS = self.d.intern_atom("WM_CLASS")
        self.NET_WM_PID = self.d.intern_atom("_NET_WM_PID")

    def get_active_window(self):
        try:
            prop = self.root.get_full_property(self.NET_ACTIVE_WINDOW, X.AnyPropertyType)
            if not prop or not prop.value:
                return None
            wid = prop.value[0]
            if not wid:
                return None
            return self.d.create_resource_object("window", wid)
        except XError:
            return None

    def _get_text_prop(self, win, atom, decode_utf8=True):
        try:
            prop = win.get_full_property(atom, X.AnyPropertyType)
            if not prop:
                return None
            val = prop.value
            if val is None:
                return None
            if isinstance(val, bytes):
                try:
                    return val.decode("utf-8" if decode_utf8 else "latin-1", errors="ignore")
                except Exception:
                    return val.decode(errors="ignore")
            if isinstance(val, str):
                return val
            if isinstance(val, (list, tuple)) and val and isinstance(val[0], int):
                try:
                    b = bytes(val)
                    return b.decode("utf-8" if decode_utf8 else "latin-1", errors="ignore")
                except Exception:
                    return None
            return None
        except (XError, BadWindow):
            return None

    def get_window_identity(self, win) -> tuple[str, list[str]]:
        title = (
            self._get_text_prop(win, self.NET_WM_NAME, True)
            or self._get_text_prop(win, self.WM_NAME, False)
            or ""
        )
        classes = []
        try:
            prop = win.get_full_property(self.WM_CLASS, X.AnyPropertyType)
            if prop and prop.value:
                raw = prop.value
                if isinstance(raw, bytes):
                    parts = raw.decode(errors="ignore").split("\x00")
                elif isinstance(raw, (list, tuple)):
                    parts = bytes(raw).decode(errors="ignore").split("\x00")
                else:
                    parts = []
                classes = [p.strip().lower() for p in parts if p.strip()]
        except (XError, BadWindow):
            pass
        return title, classes

    def get_window_pid(self, win) -> int | None:
        try:
            prop = win.get_full_property(self.NET_WM_PID, X.AnyPropertyType)
            if not prop or not prop.value:
                return None
            pid = int(prop.value[0])
            if pid <= 0:
                return None
            return pid
        except (XError, BadWindow, ValueError):
            return None


def resolve_keysym(key_str: str) -> int:
    ks = XK.string_to_keysym(key_str)
    if ks != 0:
        return ks
    if len(key_str) == 1:
        for v in (key_str.lower(), key_str.upper()):
            ks = XK.string_to_keysym(v)
            if ks != 0:
                return ks
    aliases = {
        "enter": "Return",
        "esc": "Escape",
        "del": "Delete",
        "ins": "Insert",
        "pgup": "Page_Up",
        "pgdn": "Page_Down",
        "win": "Super_L",
        "meta": "Super_L",
    }
    if key_str in aliases:
        ks = XK.string_to_keysym(aliases[key_str])
        if ks != 0:
            return ks
    if key_str.upper().startswith("KEY_") and len(key_str) == 5:
        ks = XK.string_to_keysym(key_str[-1])
        if ks != 0:
            return ks
    return 0


class KeySpec:
    def __init__(self, key_name: str, dpy, interval_s: float):
        self.key_name = key_name
        self.interval = float(interval_s)
        if self.interval <= 0:
            raise ValueError(f"Interval must be > 0 for key '{key_name}'")
        self.keysym = resolve_keysym(key_name)
        if self.keysym == 0:
            raise ValueError(f"Unrecognized key '{key_name}'. Try 'E', 'Q', 'space', 'Return', 'Left', etc.")
        self.keycode = dpy.keysym_to_keycode(self.keysym)
        if self.keycode == 0:
            raise ValueError(f"Cannot map keysym {self.keysym} for key '{key_name}' to a keycode on this layout.")
        self.next_due = time.monotonic()  # schedule first press immediately when active

    def send_once(self, dpy):
        xtest.fake_input(dpy, X.KeyPress, self.keycode)
        xtest.fake_input(dpy, X.KeyRelease, self.keycode)


class X11KeyScheduler:
    def __init__(self, app_query: str, match_mode: str, keys: list[KeySpec], display_name: str | None = None):
        self.app_query = (app_query or "").lower()
        self.match_mode = match_mode
        self.keys = keys
        self.display_name = display_name or os.environ.get("DISPLAY")

        # X connections
        self.d = display.Display(self.display_name)
        self.root = self.d.screen().root
        self.inspector = X11Inspector(self.display_name)

    def _target_is_active(self) -> bool:
        win = self.inspector.get_active_window()
        if not win:
            return False
        title, classes = self.inspector.get_window_identity(win)
        pid = self.inspector.get_window_pid(win)

        needle = self.app_query
        hay_title = (title or "").lower()
        hay_classes = [c.lower() for c in (classes or [])]

        def match_title() -> bool:
            return needle in hay_title

        def match_wmclass() -> bool:
            return any(needle in c for c in hay_classes)

        def match_exe() -> bool:
            if not pid:
                return False
            toks = get_cmdline_tokens(pid)
            if not toks:
                return False
            joined = " ".join(toks).lower()
            if needle in joined:
                return True
            for base in exe_candidates_from_tokens(toks):
                if needle in base.lower():
                    return True
            return False

        if self.match_mode == "title":
            return match_title()
        if self.match_mode == "wmclass":
            return match_wmclass()
        if self.match_mode == "exe":
            return match_exe()
        if self.match_mode == "pid":
            try:
                return pid is not None and int(needle) == pid
            except Exception:
                return False
        # any
        return match_title() or match_wmclass() or match_exe()

    def run(self, dry_run=False, poll_min=0.01, poll_max=0.1):
        LOG.info(
            "Starting: match=%s app='%s'; keys=%s",
            self.match_mode,
            self.app_query,
            ", ".join([f"{k.key_name}@{k.interval:.3f}s" for k in self.keys]),
        )
        last_state = None
        try:
            while True:
                active = self._target_is_active()
                if active != last_state:
                    LOG.info("Target active: %s", active)
                    # When becoming active, schedule immediate sends by resetting next_due
                    if active:
                        now = time.monotonic()
                        for k in self.keys:
                            k.next_due = now
                    last_state = active

                now = time.monotonic()
                if active:
                    # Send keys that are due
                    next_delta = None
                    for k in self.keys:
                        if now >= k.next_due:
                            if dry_run:
                                LOG.debug("[dry-run] would send key '%s'", k.key_name)
                            else:
                                k.send_once(self.d)
                            # schedule next occurrence; catch-up if we are late
                            while k.next_due <= now:
                                k.next_due += k.interval
                        # compute soonest next due time to sleep accordingly
                        dt = max(k.next_due - now, 0.0)
                        next_delta = dt if next_delta is None else min(next_delta, dt)
                    # Flush X events after a batch
                    if not dry_run:
                        self.d.flush()
                    # Sleep until next key is due or a small minimum
                    sleep_for = next_delta if next_delta is not None else poll_max
                    sleep_for = max(min(sleep_for, poll_max), poll_min)
                    time.sleep(sleep_for)
                else:
                    # Not active; back off a bit
                    time.sleep(0.1)
        except KeyboardInterrupt:
            LOG.info("Interrupted by user, exiting.")


def parse_key_specs(keys: list[str], default_interval: float, dpy) -> list[KeySpec]:
    """
    Parse --key KEY[:interval_seconds], allowing multiple flags.
    Example: --key E:0.2 --key Q:0.35 --key space
    If interval omitted, default_interval applies.
    """
    specs: list[KeySpec] = []
    if not keys:
        # default to E with provided default interval
        specs.append(KeySpec("E", dpy, default_interval))
        return specs
    for spec in keys:
        s = spec.strip()
        if not s:
            continue
        if ":" in s:
            key_name, interval_str = s.split(":", 1)
            key_name = key_name.strip()
            try:
                interval = float(interval_str.strip())
            except Exception:
                raise ValueError(f"Invalid interval in key spec '{s}'. Use KEY[:seconds], e.g., E:0.2")
        else:
            key_name = s
            interval = default_interval
        specs.append(KeySpec(key_name, dpy, interval))
    return specs


def discover_loop(inspector: X11Inspector, watch: bool, interval: float):
    """
    Prints info about the currently active window (title, WM_CLASS, PID, cmdline, .exe candidates),
    and a short parent chain. If watch=True, updates when the active window changes.
    """
    last_wid = None
    try:
        while True:
            win = inspector.get_active_window()
            wid = None
            if win:
                try:
                    wid = win.id
                except Exception:
                    wid = None

            if not watch or wid != last_wid:
                print("=" * 60)
                if not win:
                    print("No active X11/XWayland window detected.")
                else:
                    try:
                        title, classes = inspector.get_window_identity(win)
                        pid = inspector.get_window_pid(win)
                        print(f"Active window id: 0x{wid:x}")
                        print(f"Title: {title!r}")
                        print(f"WM_CLASS entries: {classes}")
                        print(f"_NET_WM_PID: {pid}")

                        if pid:
                            toks = get_cmdline_tokens(pid)
                            joined = " ".join(toks)
                            print(f"cmdline[{pid}]: {joined!r}")
                            exes = exe_candidates_from_tokens(toks)
                            print(f"Detected .exe candidates: {exes}")

                            # Short parent chain (up to 3)
                            chain = []
                            cur = pid
                            for _ in range(3):
                                ppid = get_ppid(cur)
                                if not ppid or ppid == 1 or ppid == cur:
                                    break
                                chain.append(ppid)
                                cur = ppid
                            if chain:
                                print("Parent chain (PPID -> COMM, first arg):")
                                for p in chain:
                                    comm = get_comm(p) or "?"
                                    t = get_cmdline_tokens(p)
                                    first = t[0] if t else "?"
                                    print(f"  {p} -> {comm} :: {first}")
                        print("\nSuggestions:")
                        if title:
                            print(f"  --match title  --app {title!r}")
                        for c in classes:
                            print(f"  --match wmclass --app {c!r}")
                        if pid:
                            toks = get_cmdline_tokens(pid)
                            exes = exe_candidates_from_tokens(toks)
                            for base in exes:
                                print(f"  --match exe    --app {base!r}")
                        if pid:
                            print(f"  --match pid    --app {pid}")
                    except Exception:
                        print("Error while reading window details:")
                        traceback.print_exc()

                last_wid = wid
                if not watch:
                    break
            time.sleep(interval)
    except KeyboardInterrupt:
        pass


def main():
    parser = argparse.ArgumentParser(
        description="Press one or more keys at fixed intervals while a specific app/window is focused (X11/XWayland). "
                    "Includes discovery to help match Proton/Wine .exe names."
    )
    parser.add_argument("--display", default=None, help="X11 DISPLAY to connect to (defaults to $DISPLAY).")
    parser.add_argument("--verbose", "-v", action="count", default=0, help="Increase verbosity (-v, -vv).")

    sub = parser.add_subparsers(dest="command")

    # Run mode
    p_run = sub.add_parser("run", help="Run the key presser")
    p_run.add_argument("--app", required=True, help="String to match. Interpreted according to --match.")
    p_run.add_argument("--match", choices=["any", "title", "wmclass", "exe", "pid"], default="any",
                       help="Match against window title, WM_CLASS, process cmdline (.exe), a PID, or any (default).")
    p_run.add_argument(
        "--key",
        action="append",
        dest="keys",
        help="Key spec as KEY[:interval_seconds]. May be given multiple times. "
             "Examples: --key E:0.2 --key Q:0.5 --key space",
    )
    p_run.add_argument(
        "--default-interval",
        type=float,
        default=0.2,
        help="Default interval (seconds) used for keys without an explicit interval. Default: 0.2",
    )
    p_run.add_argument("--hz", type=float, default=None,
                       help="Optional: set default interval via Hz (overrides --default-interval), e.g., --hz 5 -> 0.2s")
    p_run.add_argument("--dry-run", action="store_true", help="Do not press keys; just log what would happen.")

    # Discover mode
    p_disc = sub.add_parser("discover", help="Show info about the active window (title, WM_CLASS, PID, .exe candidates)")
    p_disc.add_argument("--watch", action="store_true", help="Keep running and print when active window changes.")
    p_disc.add_argument("--interval", type=float, default=0.5, help="Poll interval seconds (default 0.5).")

    args = parser.parse_args()

    if args.command is None:
        parser.print_help(sys.stderr)
        sys.exit(1)

    # Logging
    level = logging.WARNING
    if getattr(args, "verbose", 0) == 1:
        level = logging.INFO
    elif getattr(args, "verbose", 0) >= 2:
        level = logging.DEBUG
    logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", level=level)

    # Wayland check
    session_type = os.environ.get("XDG_SESSION_TYPE", "").lower()
    if session_type == "wayland" and not have_x11():
        LOG.error(
            "Wayland session detected and DISPLAY is not set.\n"
            "This X11-based method cannot inject keys under Wayland.\n"
            "Options:\n"
            "- Log into an X11 Plasma session, or ensure the target runs under XWayland (DISPLAY is set).\n"
            "- Or ask for a portal-based (xdg-desktop-portal) variant which requests user consent."
        )
        sys.exit(2)

    # Dependency check
    try:
        from Xlib import X  # noqa: F401
    except Exception:
        LOG.error("python3-Xlib is not available. Install it on Fedora with:\n  sudo dnf install python3-Xlib")
        sys.exit(3)

    # Open display early so we can resolve keys for validation
    try:
        dpy = display.Display(args.display or os.environ.get("DISPLAY"))
    except Exception as e:
        LOG.error("Failed to open X11 display: %s", e)
        sys.exit(4)

    if args.command == "discover":
        try:
            inspector = X11Inspector(args.display)
        except Exception as e:
            LOG.error("Failed to open X11 display: %s", e)
            sys.exit(4)
        discover_loop(inspector, watch=args.watch, interval=args.interval)
        return

    # command == "run"
    default_interval = args.default_interval
    if args.hz and args.hz > 0:
        default_interval = 1.0 / args.hz
    try:
        keyspecs = parse_key_specs(args.keys or [], default_interval, dpy)
    except Exception as e:
        LOG.error("Invalid --key specification: %s", e)
        sys.exit(5)

    try:
        scheduler = X11KeyScheduler(
            app_query=args.app,
            match_mode=args.match,
            keys=keyspecs,
            display_name=args.display,
        )
    except Exception as e:
        LOG.error("Failed to initialize key scheduler: %s", e)
        sys.exit(6)

    scheduler.run(dry_run=args.dry_run)


if __name__ == "__main__":
    main()
