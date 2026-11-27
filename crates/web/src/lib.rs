#![recursion_limit = "256"]
#![warn(missing_docs)]
//! Browser WebAssembly hydration entrypoint for the pure serverless edge Leptos template.
//!
//! Pages are pre-rendered to static HTML by the `ssg` binary during build time; cargo-leptos's
//! generated bootstrap script invokes [`hydrate`] to attach reactivity without any backend server.

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

/// Attaches client-side WASM reactivity to pre-rendered static edge HTML.
#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    spodeian_telemetry::init_default();
    tracing::info!("Hydrating pre-rendered edge HTML");
    leptos::mount::hydrate_body(app::App);
}
