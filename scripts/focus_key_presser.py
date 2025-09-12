#!/usr/bin/env python3
import argparse
import os
import sys
import time
import logging

# X11-only backend using python-xlib + XTest
# Works on X11 or when the target window is under XWayland with a DISPLAY available.

try:
    from Xlib import X, XK, display
    from Xlib.ext import xtest
    from Xlib.error import BadWindow, XError
except Exception:
    # We import lazily later; this helps print clearer errors if missing.
    pass

LOG = logging.getLogger("focus-key-presser")


def have_x11() -> bool:
    # A basic check for an available X Display
    return bool(os.environ.get("DISPLAY"))


class X11KeyPresser:
    def __init__(self, app_query: str, key: str, hz: float, display_name: str | None = None):
        self.app_query = app_query.lower()
        self.key = key
        self.period = 1.0 / hz if hz > 0 else 0.2
        self.display_name = display_name or os.environ.get("DISPLAY")

        # Connect to X server
        self.d = display.Display(self.display_name)
        self.root = self.d.screen().root

        # Intern common atoms
        self.NET_ACTIVE_WINDOW = self.d.intern_atom("_NET_ACTIVE_WINDOW")
        self.NET_WM_NAME = self.d.intern_atom("_NET_WM_NAME")
        self.UTF8_STRING = self.d.intern_atom("UTF8_STRING")
        self.WM_NAME = self.d.intern_atom("WM_NAME")
        self.WM_CLASS = self.d.intern_atom("WM_CLASS")

        # Resolve the keysym/code once up front if possible
        self.keysym = self._resolve_keysym(self.key)
        if self.keysym == 0:
            raise ValueError(f"Unrecognized key '{self.key}'. Try names like 'E', 'space', 'Return', 'Left', etc.")
        self.keycode = self.d.keysym_to_keycode(self.keysym)
        if self.keycode == 0:
            raise ValueError(f"Cannot map keysym {self.keysym} for key '{self.key}' to a keycode on this layout.")

    @staticmethod
    def _resolve_keysym(key_str: str) -> int:
        """
        Resolve a key string to an X keysym integer.
        Accepts single characters and common key names ('space', 'Return', 'Left', etc.).
        """
        # Try as given
        ks = XK.string_to_keysym(key_str)
        if ks != 0:
            return ks
        # Try lowercase for single letters
        if len(key_str) == 1:
            ks = XK.string_to_keysym(key_str.lower())
            if ks != 0:
                return ks
            ks = XK.string_to_keysym(key_str.upper())
            if ks != 0:
                return ks
        # Common aliases
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
        # Some users might pass e.g. "KEY_E"; strip prefix
        if key_str.upper().startswith("KEY_") and len(key_str) == 5:
            ks = XK.string_to_keysym(key_str[-1])
            if ks != 0:
                return ks
        return 0

    def _get_active_window(self):
        try:
            prop = self.root.get_full_property(self.NET_ACTIVE_WINDOW, X.AnyPropertyType)
            if not prop or not prop.value:
                return None
            win_id = prop.value[0]
            if win_id == 0:
                return None
            return self.d.create_resource_object("window", win_id)
        except XError:
            return None

    def _get_window_text_property(self, win, atom, decode_utf8=True):
        try:
            prop = win.get_full_property(atom, X.AnyPropertyType)
            if not prop:
                return None
            val = prop.value
            if val is None:
                return None
            if isinstance(val, bytes):
                if decode_utf8:
                    try:
                        return val.decode("utf-8", errors="ignore")
                    except Exception:
                        return val.decode(errors="ignore")
                else:
                    try:
                        return val.decode(errors="ignore")
                    except Exception:
                        return None
            if isinstance(val, str):
                return val
            # Some properties return arrays of 8-bit ints
            if isinstance(val, (list, tuple)) and val and isinstance(val[0], int):
                try:
                    b = bytes(val)
                    return b.decode("utf-8" if decode_utf8 else "latin-1", errors="ignore")
                except Exception:
                    return None
            return None
        except (XError, BadWindow):
            return None

    def _get_window_identity_strings(self, win):
        """
        Returns a tuple (title, classes) where:
         - title is string or ''
         - classes is a list of class/instance strings (lowercased)
        """
        title = (
            self._get_window_text_property(win, self.NET_WM_NAME, decode_utf8=True)
            or self._get_window_text_property(win, self.WM_NAME, decode_utf8=False)
            or ""
        )

        wm_class_raw = None
        try:
            prop = win.get_full_property(self.WM_CLASS, X.AnyPropertyType)
            if prop and prop.value:
                wm_class_raw = prop.value
        except (XError, BadWindow):
            wm_class_raw = None

        classes = []
        if wm_class_raw is not None:
            # WM_CLASS is two consecutive null-terminated strings: instance\0class\0
            if isinstance(wm_class_raw, bytes):
                parts = wm_class_raw.decode(errors="ignore").split("\x00")
                classes = [p.strip().lower() for p in parts if p]
            elif isinstance(wm_class_raw, (list, tuple)):
                try:
                    b = bytes(wm_class_raw)
                    parts = b.decode(errors="ignore").split("\x00")
                    classes = [p.strip().lower() for p in parts if p]
                except Exception:
                    pass

        return title, classes

    def _target_is_active(self) -> bool:
        win = self._get_active_window()
        if not win:
            return False
        title, classes = self._get_window_identity_strings(win)
        needle = self.app_query
        hay_title = (title or "").lower()
        if needle in hay_title:
            return True
        for c in classes:
            if needle in c:
                return True
        return False

    def _send_key_once(self):
        # Press + Release
        xtest.fake_input(self.d, X.KeyPress, self.keycode)
        xtest.fake_input(self.d, X.KeyRelease, self.keycode)
        self.d.flush()

    def run(self, dry_run=False):
        LOG.info("Starting key presser: app='%s', key='%s', every %.3fs", self.app_query, self.key, self.period)
        last_log_state = None
        try:
            while True:
                active = self._target_is_active()
                if active != last_log_state:
                    LOG.info("Target active: %s", active)
                    last_log_state = active
                if active:
                    if dry_run:
                        LOG.debug("[dry-run] would send key '%s'", self.key)
                    else:
                        self._send_key_once()
                time.sleep(self.period)
        except KeyboardInterrupt:
            LOG.info("Interrupted by user, exiting.")


def main():
    parser = argparse.ArgumentParser(
        description="Send a key at a fixed frequency while a specific application/window is focused (X11/XWayland)."
    )
    parser.add_argument("--app", required=True, help="Application identifier to match (substring of title or WM_CLASS).")
    parser.add_argument("--key", default="E", help="Key to press (e.g., E, space, Return, Left). Default: E")
    parser.add_argument("--hz", type=float, default=5.0, help="Times per second to press the key. Default: 5.0")
    parser.add_argument("--display", default=None, help="X11 DISPLAY to connect to (defaults to $DISPLAY).")
    parser.add_argument("--dry-run", action="store_true", help="Do not press keys; just log what would happen.")
    parser.add_argument("--verbose", "-v", action="count", default=0, help="Increase verbosity (-v, -vv).")

    args = parser.parse_args()

    # Configure logging
    level = logging.WARNING
    if args.verbose == 1:
        level = logging.INFO
    elif args.verbose >= 2:
        level = logging.DEBUG
    logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", level=level)

    # Environment checks and guidance
    session_type = os.environ.get("XDG_SESSION_TYPE", "").lower()
    if session_type == "wayland" and not have_x11():
        LOG.error(
            "Wayland session detected and DISPLAY is not set.\n"
            "This X11-based method cannot inject keys into other applications under Wayland.\n"
            "Options:\n"
            "- Log into an X11 Plasma session, or run the target app under XWayland (with DISPLAY).\n"
            "- Or ask me for a portal-based (xdg-desktop-portal RemoteDesktop) variant that requests user consent.\n"
        )
        sys.exit(2)

    # Lazy import check for python-xlib
    try:
        from Xlib import X  # noqa: F401
    except Exception:
        LOG.error(
            "python3-Xlib is not available. Install it on Fedora with:\n"
            "  sudo dnf install python3-Xlib"
        )
        sys.exit(3)

    if args.hz <= 0:
        LOG.error("Hz must be > 0")
        sys.exit(4)

    try:
        presser = X11KeyPresser(args.app, args.key, args.hz, args.display)
    except Exception as e:
        LOG.error("Failed to initialize X11 backend: %s", e)
        sys.exit(5)

    presser.run(dry_run=args.dry_run)


if __name__ == "__main__":
    main()