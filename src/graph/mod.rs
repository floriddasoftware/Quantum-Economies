// src/graph/mod.rs

pub mod slotmap;
pub mod frame;
pub mod scale;
pub mod permdimensions2d;
pub mod qpdimensions2d;

// =============================================================================
// LOCAL GRAPH STORAGE / EVENT MODEL
// =============================================================================
//
// slotmap.rs is the canonical local-crate owner of:
//
//     SurfaceId
//     EventId
//     QtmEvent
//     QuantumTransaction
//     SurfaceCoordinate
//     SurfaceNode
//     QtmGraph
//
// Keep these names sourced from the local `slotmap` module.
//
pub use self::slotmap::{
    EventId,
    QtmEvent,
    QtmGraph,
    QuantumTransaction,
    SurfaceCoordinate,
    SurfaceId,
    SurfaceNode,
};

// =============================================================================
// GRAPH FRAME
// =============================================================================
//
// frame.rs contains the higher-level frame/observer structures.
// Do not glob-re-export it here because several names overlap with
// slotmap.rs.
//
pub use self::frame::{
    EventMap,
    QtmEdge,
    Direction,
    Depth
};

// =============================================================================
// SCALE
// =============================================================================

pub use self::scale::Scale;

// =============================================================================
// 2D PROTOCOL DOMAINS
// =============================================================================
//
// Each protocol keeps its complete independent 2D behavior.
//
//     permdimensions2d = authoritative PERM 2D domain
//     qpdimensions2d   = realized QuantPerm 2D domain
//
// main.rs can import both independently and compose them into the
// combined 4D observer.
//
pub use self::permdimensions2d::*;
pub use self::qpdimensions2d::*;