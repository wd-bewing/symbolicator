//! FIPS 140-3 support: load OpenSSL FIPS provider at startup when FIPS_MODE=1.
//!
//! When the `fips` Cargo feature is enabled and `FIPS_MODE=1` is set in the environment,
//! the OpenSSL FIPS provider is loaded before any other crypto or TLS. Outbound HTTPS
//! (reqwest) then uses OpenSSL's FIPS provider when built with `--features fips`.

use std::env;
use std::sync::Once;

const FIPS_MODE_ENV: &str = "FIPS_MODE";
static FIPS_LOADED: Once = Once::new();

/// Loads the OpenSSL FIPS provider at startup when FIPS_MODE=1. Must be called
/// before any TLS or other crypto. When the `fips` feature is disabled, this is a no-op.
///
/// Returns Ok(()) if FIPS is not requested, or if the provider was loaded successfully.
/// Returns Err when FIPS_MODE=1 and the provider failed to load.
#[cfg(feature = "fips")]
pub fn ensure_fips_loaded() -> Result<(), String> {
    if env::var(FIPS_MODE_ENV).as_deref() != Ok("1") {
        return Ok(());
    }

    let mut err = Ok(());
    FIPS_LOADED.call_once(|| {
        match openssl::provider::Provider::load(None, "fips") {
            Ok(provider) => {
                std::mem::forget(provider);
                tracing::info!("FIPS 140-3: OpenSSL FIPS provider loaded successfully");
                err = Ok(());
            }
            Err(e) => {
                let msg = e
                    .errors()
                    .iter()
                    .filter_map(|e| e.reason().map(String::from))
                    .collect::<Vec<_>>()
                    .join("; ");
                err = Err(format!("Failed to load OpenSSL FIPS provider: {msg}"));
            }
        }
    });

    err
}

#[cfg(not(feature = "fips"))]
pub fn ensure_fips_loaded() -> Result<(), String> {
    Ok(())
}
