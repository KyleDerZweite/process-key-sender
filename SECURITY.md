# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.2.x   | Yes       |
| 0.1.x   | No        |

## Reporting a Vulnerability

Please do not file public issues for security reports.

- Email: [info@kylehub.dev](mailto:info@kylehub.dev)
- Include: affected version, reproduction steps, impact, and any suggested mitigation

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
