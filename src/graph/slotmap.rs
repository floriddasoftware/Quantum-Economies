use crate::graph::Scale;
use std::collections::HashMap;

// =============================================================================
// GRAPH IDS
// =============================================================================
//
// Graph identities are protocol-sized 32-byte values.
//
// No external `slotmap` crate is required.
// =============================================================================

pub type SurfaceId = [u8; 32];
pub type EventId = [u8; 32];

// =============================================================================
// QTM EVENT
// =============================================================================

#[derive(Debug, Clone)]
pub struct QtmEvent {
    pub commitment: [u8; 32],
    pub activation: u64,
    pub network: u128,
    pub coordinate: [u8; 32],
    pub delta: u128,
    pub sigma: u128,
    pub dimension: u64,
    pub scale: Scale,
}

pub type QuantumTransaction = QtmEvent;

// =============================================================================
// SURFACE GEOMETRY
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct SurfaceCoordinate {
    pub dimension: u64,
    pub sigma: u128,
}

// =============================================================================
// SURFACE NODE
// =============================================================================

#[derive(Debug, Clone)]
pub struct SurfaceNode {
    pub coordinate: [u8; 32],
    pub surface: SurfaceCoordinate,
    pub scale: Scale,
    pub total_network: u128,
    pub events: Vec<EventId>,
    pub inbound: Vec<SurfaceId>,
    pub outbound: Vec<SurfaceId>,
}

// =============================================================================
// GRAPH / EVENT STORE
// =============================================================================

#[derive(Debug, Default)]
pub struct QtmGraph {
    pub events: HashMap<EventId, QtmEvent>,
    pub surfaces: HashMap<SurfaceId, SurfaceNode>,
    pub commitments: HashMap<[u8; 32], EventId>,
    pub activation_network: HashMap<u64, u128>,
    pub coordinates: HashMap<[u8; 32], SurfaceId>,
    pub sigma_index: HashMap<u128, Vec<SurfaceId>>,
}

// =============================================================================
// GRAPH IMPLEMENTATION
// =============================================================================

impl QtmGraph {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // INSERT QTM EVENT
    // -------------------------------------------------------------------------

    pub fn insert(
        &mut self,
        event: QtmEvent,
    ) -> (SurfaceId, EventId) {
        let coordinate = event.coordinate;
        let commitment = event.commitment;
        let event_id = commitment;

        let activation = event.activation;
        let network = event.network;
        let dimension = event.dimension;
        let sigma = event.sigma;
        let scale = event.scale;

        self.events.insert(
            event_id,
            event,
        );

        self.commitments.insert(
            commitment,
            event_id,
        );

        self.activation_network.insert(
            activation,
            network,
        );

        // ---------------------------------------------------------------------
        // Existing surface
        // ---------------------------------------------------------------------

        if let Some(&surface_id) =
            self.coordinates.get(&coordinate)
        {
            let surface = self
                .surfaces
                .get_mut(&surface_id)
                .expect(
                    "coordinate index contains invalid SurfaceId",
                );

            surface.total_network =
                surface
                    .total_network
                    .saturating_add(network);

            surface.events.push(event_id);

            return (
                surface_id,
                event_id,
            );
        }

        // ---------------------------------------------------------------------
        // New surface
        // ---------------------------------------------------------------------

        let surface_id = coordinate;

        let surface = SurfaceNode {
            coordinate,
            surface: SurfaceCoordinate {
                dimension,
                sigma,
            },
            scale,
            total_network: network,
            events: vec![event_id],
            inbound: Vec::new(),
            outbound: Vec::new(),
        };

        self.surfaces.insert(
            surface_id,
            surface,
        );

        self.coordinates.insert(
            coordinate,
            surface_id,
        );

        self.sigma_index
            .entry(sigma)
            .or_default()
            .push(surface_id);

        (
            surface_id,
            event_id,
        )
    }
}