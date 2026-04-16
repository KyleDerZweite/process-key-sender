# CLI Reference

The CLI supports quick single-key automation and config file loading. More advanced automation options live in [configuration.md](configuration.md).

## Usage

```text
pks [OPTIONS]
```

## Options

- `-c, --config <FILE>`: load a JSON config file.
- `-p, --process <PROCESS>`: target process name.
- `-k, --key <KEY>`: key to send.
- `-i, --interval <DURATION>`: interval between sends. Default: `1000ms`.
- `--max-retries <COUNT>`: process lookup attempts. Default: `10`.
- `--save-config <FILE>`: save the current CLI arguments as a config file and exit.
- `-v, --verbose`: enable verbose output.
- `-h, --help`: print help.
- `-V, --version`: print version.

## Examples

Run a single key:

```bash
pks --process notepad.exe --key space
```

Run with a custom interval:

```bash
pks --process game.exe --key r --interval 1500ms
```

Load a config file:

```bash
pks --config examples/configs/advanced-config.json
```

Save a config file from CLI arguments:

```bash
pks --process notepad.exe --key space --save-config my-config.json
```

## Notes

- CLI mode creates a single repeating key sequence.
- Sequence playback, repeat counts, independent timers, `pause_hotkey`, and `restore_focus` are configured through JSON files.
