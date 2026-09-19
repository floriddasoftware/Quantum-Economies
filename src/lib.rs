//! QTM — Quantum Transaction Manifold Core
//! Ledgerless, observer-relative, deterministic state engine.

pub mod dash;
pub mod graph;

// -----------------------------------------------------------------------------
// Dashboard / observer layer
// -----------------------------------------------------------------------------

pub use dash::{
    Dashboard,
    DashboardBuilder,
    DashboardExt,
    DashboardView,
};

// -----------------------------------------------------------------------------
// Graph / geometric observer layer
//
// Each protocol dimension owns its complete 2D behavior:
//
//     permdimensions2d  -> PERM 2D domain
//     qpdimensions2d    -> QuantPerm 2D domain
//
// main.rs imports these independent projections and composes them into
// the combined 4D observer.
//
// Do not duplicate the 2D behavior here.
// -----------------------------------------------------------------------------

pub use graph::{
    permdimensions2d,
    qpdimensions2d,
};

// -----------------------------------------------------------------------------
// Start a default QTM system
// -----------------------------------------------------------------------------

pub fn bootstrap() -> Dashboard {
    Dashboard::new()
}