# Security Policy

## Supported Versions

The project is archived. Security support has ended for all versions.

## Project Security Notes

- The project does not perform network communication or telemetry.
- Configuration files are plain JSON and should not contain secrets.
- Windows key sending changes window focus by design unless `restore_focus` is enabled.
- Unix/Linux builds are supported for development, but runtime key sending is not implemented there.

## User Responsibilities

- Review configs before running them.
- Test automation against non-critical applications first.
- Avoid using the tool with online games or services that prohibit automation.
- Run with the minimum privileges needed.
