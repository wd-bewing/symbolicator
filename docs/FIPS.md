# FIPS 140-3 Compatibility

Symbolicator can be built and run with FIPS 140-3 compatible cryptography. In this mode, outbound HTTPS (to symbol servers) uses OpenSSL's FIPS-validated provider via the `native-tls` backend; the OpenSSL FIPS provider is loaded at startup when `FIPS_MODE=1` is set.

## Requirements

- **OpenSSL 3.x** built with FIPS support (e.g. OpenSSL 3.0.0+ with the FIPS provider).
- At build time: OpenSSL development headers and library in a FIPS configuration, or a FIPS-built OpenSSL install pointed to by `OPENSSL_DIR` (see below).
- At runtime: The FIPS provider must be loadable; set `OPENSSL_MODULES` to the directory containing the FIPS provider shared library (e.g. `openssl-modules` or `ossl-modules`).

## Building Symbolicator with FIPS

1. **Build or install OpenSSL 3.x with FIPS** (see your platform or distribution documentation).

2. **Build the symbolicator binary with the `fips` feature**

   Use `--no-default-features --features fips` so that the default rustls TLS backend is replaced by native-tls (OpenSSL):

   ```bash
   cargo build --release --no-default-features --features fips -p symbolicator
   ```

   If OpenSSL is not in a default system path:

   ```bash
   export OPENSSL_DIR=/opt/openssl-fips
   export OPENSSL_LIB_DIR=/opt/openssl-fips/lib
   export OPENSSL_INCLUDE_DIR=/opt/openssl-fips/include
   cargo build --release --no-default-features --features fips -p symbolicator
   ```

3. **Runtime**

   Ensure the FIPS provider is available when starting Symbolicator:

   ```bash
   export OPENSSL_MODULES=/opt/openssl-fips/lib/ossl-modules
   export FIPS_MODE=1
   ./target/release/symbolicator run -c /etc/symbolicator/config.yml
   ```

   If the provider fails to load when `FIPS_MODE=1` is set, the process exits at startup with an error.

## What changes in FIPS mode

- **TLS**: Outbound HTTPS (reqwest) uses the `native-tls` backend (OpenSSL) instead of rustls. When linked against a FIPS-built OpenSSL and with the FIPS provider loaded, TLS uses FIPS-approved algorithms.
- **Startup**: When `FIPS_MODE=1`, the process loads the OpenSSL FIPS provider before any other crypto or TLS.
- **Inbound server**: The current HTTP server is HTTP only. If HTTPS is added in the future, that path must use an OpenSSL-based acceptor when building with FIPS.

## Notes

- **Default build**: The default build uses rustls and does **not** enable FIPS. Only a build with `--no-default-features --features fips -p symbolicator` loads the FIPS provider when `FIPS_MODE=1` and uses OpenSSL for TLS.
- **Validation**: This setup uses a FIPS-validated cryptographic module (OpenSSL's FIPS provider). Symbolicator as an application is not itself FIPS 140-3 validated; formal product validation would require the CMVP process.
- **Platform**: FIPS builds are typically used on Linux with a FIPS-built OpenSSL.
