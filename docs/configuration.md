# Configuration Guide

`pks` supports richer automation via JSON config files than via CLI flags alone. Use config files for key sequences, independent timers, custom pause hotkeys, repeat control, and focus-restoration behavior.

## Example Files

Tracked examples live in `examples/configs/`:

- `config.json`: basic independent timers
- `single-key-config.json`: single repeating key
- `sequence-config.json`: ordered sequence with repeat count
- `advanced-config.json`: multi-key config with explicit `restore_focus`

Run one directly:

```bash
pks --config examples/configs/sequence-config.json
```

## Schema

```json
{
  "process_name": "target-process.exe",
  "key_sequence": [],
  "independent_keys": [],
  "max_retries": 10,
  "pause_hotkey": "ctrl+alt+r",
  "verbose": false,
  "loop_sequence": true,
  "repeat_count": 0,
  "restore_focus": true
}
```

## Fields

- `process_name`: required process name to match.
- `key_sequence`: ordered actions using `key` plus `interval_after`.
- `independent_keys`: timer-based actions using `key` plus `interval`.
- `max_retries`: process lookup attempts before giving up.
- `pause_hotkey`: global toggle for pause/resume.
- `verbose`: print each send action.
- `loop_sequence`: repeat the sequence indefinitely.
- `repeat_count`: run a sequence a fixed number of times. `0` means unlimited.
- `restore_focus`: on Windows, restore the previously focused window after sending keys.

Only one mode may be active at a time: use `key_sequence` or `independent_keys`, not both.

## Time Formats

- `1000ms`
- `1s`
- `1m`
- `1000` for milliseconds

## Supported Keys

- Letters: `a-z`
- Numbers: `0-9`
- Function keys: `f1` through `f12`
- Special keys: `space`, `enter`, `return`, `tab`, `escape`, `esc`, `backspace`, `delete`, `insert`
- Navigation keys: `home`, `end`, `pageup`, `pagedown`
- Arrow keys: `left`, `right`, `up`, `down`
- Standalone modifiers: `ctrl`, `control`, `shift`, `alt`
- Combinations: `ctrl+s`, `alt+tab`, `ctrl+shift+f10`

## Sequence Example

```json
{
  "process_name": "notepad.exe",
  "key_sequence": [
    { "key": "1", "interval_after": "500ms" },
    { "key": "2", "interval_after": "500ms" },
    { "key": "space", "interval_after": "1s" }
  ],
  "loop_sequence": false,
  "repeat_count": 3
}
```

## Independent Timer Example

```json
{
  "process_name": "editor.exe",
  "independent_keys": [
    { "key": "ctrl+s", "interval": "30s" },
    { "key": "tab", "interval": "1500ms" }
  ],
  "pause_hotkey": "ctrl+shift+p",
  "restore_focus": false
}
```
