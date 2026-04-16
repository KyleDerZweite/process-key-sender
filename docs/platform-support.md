# Platform Support

## Support Matrix

| Capability | Windows | Unix/Linux |
| ---------- | ------- | ---------- |
| Build from source | Yes | Yes |
| Parse config files | Yes | Yes |
| Validate key names | Yes | Yes |
| Process discovery | Yes | Yes |
| Global hotkey setup | Yes | Environment-dependent |
| Send keys to target process | Yes | No |

## Details

### Windows

Windows is the supported runtime platform for the full automation workflow. The project can locate the target process, send keys, and optionally restore the previously focused window after each send.

### Unix/Linux

Unix/Linux is supported for development, config validation, and most automated tests. Runtime key sending is not implemented, so attempts to send keys will fail with a clear unsupported-platform error.

The `global-hotkey` crate may also depend on local desktop or X11 availability. Build success does not guarantee that hotkey registration will work in every Unix/Linux environment.
