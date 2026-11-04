# Security Policy

## Project status

Hakorune is an experimental language implementation. It is under active
development and has not been hardened for hostile environments. Please read
the limitations below before relying on it.

## Supported versions

Only the latest `main` is supported. There are no maintained release branches.

## Reporting a vulnerability

Please report suspected vulnerabilities privately through GitHub's
**private vulnerability reporting**: open the repository's **Security** tab and
choose **Report a vulnerability**.

Please do not open a public issue or pull request for a vulnerability, and
do not include working exploit details in public places.

This is a small project. We read reports and will respond as we can, but we do
not promise a response or fix deadline.

## Known limitations

These are current, known limits rather than reportable vulnerabilities:

- **Experimental stage.** Interfaces, behavior, and internals change without
  notice.
- **Ownership safety is partly connected.** The ownership model is still being
  wired through the compiler; do not assume ownership-related guarantees hold
  across the whole language yet.
- **Not suited to untrusted input.** Do not run untrusted Hakorune programs or
  feed untrusted input to the compiler or runtime, and do not treat it as a
  sandbox.
- **Plugins are loaded without integrity checks.** Plugin libraries named in
  the configuration (`nyash.toml`) are loaded dynamically (`dlopen` via
  `libloading`) after only a path-existence check. There is no hash,
  signature, or other integrity verification, and a loaded plugin runs with
  the full privileges of the host process. Likewise, the native AOT FFI library
  can be selected through the `HAKO_AOT_FFI_LIB` environment variable.
  Only use plugins and configuration you trust.
