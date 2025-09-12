# Focus Key Presser (Fedora 42 KDE Plasma, X11/XWayland)

Press one or more keys at fixed intervals while a specific window/app is focused. Supports matching by window title, WM_CLASS, executable name from Proton/Wine (`.exe`), or PID. Includes a discovery mode so you can see exactly what to match.

This is designed to work under default enforcing SELinux with no policy changes.

## Why this approach?

- Under Wayland, generic key injection into other apps is restricted by design.
- Many Proton/Wine games run under XWayland, exposing an X11 window where XTest is allowed.
- This tool uses X11’s XTest extension via `python-xlib`, requires no root, and avoids `/dev/uinput`.

## Requirements

- Fedora (KDE Plasma), X11 or XWayland session for the target window.
- Python 3
- python-xlib:
  ```bash
  sudo dnf install python3-Xlib
  ```

## SELinux

- Works with the default enforcing policy out of the box.
- Operations:
  - Connect to your user X server (`$DISPLAY`) and send XTest events.
  - Read `/proc/<pid>/cmdline` for processes you own (to detect `.exe` names).
- No SELinux booleans, context changes, or `audit2allow` rules are needed.
- If you encounter AVC denials (unexpected), collect them with `ausearch -m avc -ts recent` and share; the script itself is designed to avoid any.

## Wayland vs X11

- This tool injects keys via X11. It works if the target window is running under X11/XWayland.
- If your session is Wayland and the target is Wayland-native, this won’t inject keys.
- Options if you must stay Wayland-native:
  - Run the app under XWayland.
  - Or use an xdg-desktop-portal RemoteDesktop approach (prompts for user consent). Ask and we can provide a portal-based variant.

## Usage

```
python3 focus_key_presser.py discover [--watch]
python3 focus_key_presser.py run --match {any|title|wmclass|exe|pid} --app <STRING> \
  [--key KEY[:interval_s]]... [--default-interval SEC | --hz HZ] [--dry-run] [-v|-vv]
```

- `discover`: Prints information about the currently focused X11/XWayland window:
  - Title, WM_CLASS, `_NET_WM_PID`, full `cmdline`, and detected `.exe` basenames.
  - Use `--watch` to update when focus changes.

- `run`:
  - `--match` chooses what `--app` matches:
    - `title` → substring of window title
    - `wmclass` → substring of WM_CLASS (instance/class)
    - `exe` → substring of the focused process cmdline and `.exe` basenames (Proton/Wine)
    - `pid` → exact PID of the focused window
    - `any` (default) → title or wmclass or exe
  - `--key KEY[:interval_s]` may be given multiple times to send multiple keys, each at its own interval.
    - Examples: `--key E:0.2 --key Q:0.5 --key space`
    - If interval is omitted, `--default-interval` applies.
  - `--default-interval` sets the default seconds between key presses for keys without explicit intervals (default 0.2).
  - `--hz` optionally sets default rate as presses per second (overrides `--default-interval`). For example, `--hz 5` means 0.2 seconds.

### Examples

- Discover what to match:
  ```bash
  python3 focus_key_presser.py discover --watch
  ```
  Focus your game window; you’ll see title, WM_CLASS, PID, cmdline, and `.exe` candidates.

- Press E and Q every 0.2 seconds while a specific PID is focused:
  ```bash
  python3 focus_key_presser.py run --match pid --app 20537 --key E:0.2 --key Q:0.2
  ```

- Press different keys at different intervals while matching by `.exe` name (Proton/Wine):
  ```bash
  python3 focus_key_presser.py run --match exe --app "Shape of Dreams.exe" --key E:0.2 --key Q:0.35
  ```

- Same interval for multiple keys using a default:
  ```bash
  python3 focus_key_presser.py run --match wmclass --app "steam_app_123456" \
    --default-interval 0.2 --key E --key Q --key space
  ```

- Dry run with verbose logs (no key injection):
  ```bash
  python3 focus_key_presser.py run --match exe --app "Shape of Dreams.exe" \
    --key E:0.2 --key Q:0.2 --dry-run -vv
  ```

## Notes

- Key names are X keysyms (case-insensitive). Examples: `E`, `Q`, `space`, `Return`, `Left`, `Right`, `Escape`.
- For browser-based titles, `--match title` usually works best; for native apps, `--match wmclass`; for Proton/Wine, `--match exe` is often most reliable.
- Many Proton games expose `_NET_WM_PID`, so PID matching works if that PID owns the focused window.
- The script polls the active window frequently and schedules each key independently to maintain your requested intervals.
