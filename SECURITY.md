# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| latest  | ✅        |
| < 1.0   | ❌        |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Please report security issues by email to: **aguado.mathis@pm.me**

Include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Your suggested fix (optional)

You will receive a response within 72 hours. If the issue is confirmed, a patch will be released as soon as possible. You will be credited in the changelog unless you prefer to remain anonymous.

## Scope

Verto is a local-only desktop application. It does not communicate with any remote server. The main attack surfaces are:

- Malicious input files triggering vulnerabilities in FFmpeg, Pandoc, or the Rust `image` crate
- Arbitrary file write via path traversal in output path handling
- Binary sidecar integrity (FFmpeg/Pandoc bundled binaries)

Out of scope: social engineering, physical access.
