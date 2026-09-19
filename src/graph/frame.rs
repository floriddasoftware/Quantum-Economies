// src/graph/frame.rs

use quantom_value::{
    Heritage,
    QuantPerm,
    Retain,
};

use std::{collections::{BTreeMap, HashMap}, time::SystemTime};

use super::scale::Scale;

// =============================================================================
// GRAPH IDENTITIES
// =============================================================================

pub type EventId = [u8; 32];
pub type SurfaceId = [u8; 32];
pub type EdgeId = [u8; 32];

// =============================================================================
// SURFACE COORDINATE
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct SurfaceCoordinate {
    pub dimension: u64,
    pub sigma: u128,
}

// =============================================================================
// QTM EVENT
//
// Heritage is the authentic transition receipt.
// Qtm remains a derived projection and is deliberately not stored here.
// =============================================================================

pub struct QtmEvent {
    pub heritage: Heritage,
    pub activation: u64,
    pub scale: Scale,
}

// =============================================================================
// SURFACE NODE
// =============================================================================

#[derive(Debug, Clone)]
pub struct SurfaceNode {
    pub coordinate: [u8; 32],
    pub surface: SurfaceCoordinate,
    pub scale: Scale,

    pub structural_value: u128,
    pub total_network: u128,
    pub visits: u64,

    pub activations: Vec<u64>,
    pub events: Vec<EventId>,

    pub inbound: Vec<EdgeId>,
    pub outbound: Vec<EdgeId>,
}

// =============================================================================
// GRAPH EDGE
// =============================================================================

#[derive(Debug, Clone)]
pub struct QtmEdge {
    pub from: SurfaceId,
    pub to: SurfaceId,

    pub net_work: u128,
    pub sigma_delta: i128,
    pub dimension_delta: i128,

    pub commitment_changed: bool,
    pub coordinate_changed: bool,
}

// =============================================================================
// EVENT MAP
//
// Heritage is the authentic transition receipt.
//
// EventMap temporarily owns Heritage while exposing the canonical event
// projection. No Heritage is cloned and no authenticity is reconstructed
// from independently supplied parameters.
// =============================================================================

pub struct EventMap {
    pub heritage: Heritage,
    pub activation: u64,
    pub scale: Scale,
    pub runtime: SystemTime,
}

impl EventMap {
    #[inline(always)]
    pub fn new(
        heritage: Heritage,
        scale: Scale,
        runtime: SystemTime,
    ) -> Self {
        let activation =
            heritage.state.activations();

        Self {
            heritage,
            activation,
            scale,
            runtime,
        }
    }

    #[inline(always)]
    pub fn now(
        heritage: Heritage,
        scale: Scale,
    ) -> Self {
        Self::new(
            heritage,
            scale,
            SystemTime::now(),
        )
    }

    // -------------------------------------------------------------------------
    // EVENT RUNTIME
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn runtime(
        &self,
    ) -> SystemTime {
        self.runtime
    }

    // -------------------------------------------------------------------------
    // AUTHENTIC HERITAGE
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn heritage(
        &self,
    ) -> &Heritage {
        &self.heritage
    }

    // -------------------------------------------------------------------------
    // TRANSITION VALUES
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn tau(
        &self,
    ) -> u128 {
        self.heritage.transition.tau
    }

    #[inline(always)]
    pub fn delta(
        &self,
    ) -> u128 {
        self.heritage.transition.delta
    }

    #[inline(always)]
    pub fn gross_work(
        &self,
    ) -> u128 {
        self.heritage.transition.gross_work
    }

    #[inline(always)]
    pub fn net_work(
        &self,
    ) -> u128 {
        self.heritage.transition.net_work
    }

    #[inline(always)]
    pub fn mirror_bytes(
        &self,
    ) -> &[u8; 32] {
        &self.heritage.transition.mirror_bytes
    }

    // -------------------------------------------------------------------------
    // POST-TRANSITION STATE
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn coordinate(
        &self,
    ) -> u64 {
        self.heritage.state.dimension()
    }

    #[inline(always)]
    pub fn dimension(
        &self,
    ) -> u64 {
        self.heritage.state.dimension()
    }

    #[inline(always)]
    pub fn structural_value(
        &self,
    ) -> u128 {
        self.heritage.state.structural_value()
    }

    #[inline(always)]
    pub fn retained_mass(
        &self,
    ) -> u128 {
        self.heritage.state.retained_mass()
    }

    #[inline(always)]
    pub fn activations(
        &self,
    ) -> u64 {
        self.heritage.state.activations()
    }

    // -------------------------------------------------------------------------
    // EVENT VALUES
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn activation(
        &self,
    ) -> u64 {
        self.activation
    }

    #[inline(always)]
    pub fn scale(
        &self,
    ) -> Scale {
        self.scale
    }

    // -------------------------------------------------------------------------
    // OWNERSHIP RETURN
    //
    // EventMap is only a temporary event projection boundary.
    //
    // Once the event has been consumed, return the authentic Heritage to the
    // caller. No clone is required.
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn into_heritage(
        self,
    ) -> Heritage {
        self.heritage
    }
}

// =============================================================================
// DIRECTION
//
// Direction is the graph-layer transition boundary.
//
// It owns the authentic EventMap after a transition.
// Dashboard never owns Heritage and never performs transitions.
//
//     QuantPerm + Retain
//              │
//              ▼
//        Direction::transit
//              │
//              ▼
//           Heritage
//              │
//              ▼
//          EventMap
//              │
//              ├────────► Dashboard
//              │
//              └────────► other observers
// =============================================================================

pub struct Direction {
    pub event: EventMap,
}

impl Direction {

    #[inline(always)]
    pub fn realize_depth(
        manifold: &QuantPerm,
        depth: &Depth,
    ) -> Retain {
        let mass = depth
            .objects()
            .fold(
                0u128,
                |acc, (_, _, retain)| {
                    acc.saturating_add(retain.mass)
                },
            );
    
        manifold.retain(
            mass,
            manifold.dimension(),
        )
    }
    
    
    // -------------------------------------------------------------------------
    // CANONICAL TRANSITION
    // -------------------------------------------------------------------------
 
    pub fn transit(
        manifold: QuantPerm,
        depth: &Depth,
        provided_seed: Option<&[u8]>,
    ) -> Heritage {
        let retain = Self::realize_depth(&manifold, depth);
        manifold.transition(&retain, provided_seed)
    }


    // -------------------------------------------------------------------------
    // EXILE
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn exile(
        manifold: QuantPerm,
    ) -> Heritage {
        QuantPerm::exile(manifold)
    }

    // -------------------------------------------------------------------------
    // EVENT CONSTRUCTION
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn new(
        heritage: Heritage,
        scale: Scale,
    ) -> Self {
        Self {
            event: EventMap::now(
                heritage,
                scale,
            ),
        }
    }

    #[inline(always)]
    pub fn from_heritage(
        heritage: Heritage,
        scale: Scale,
    ) -> Self {
        Self::new(
            heritage,
            scale,
        )
    }

    // -------------------------------------------------------------------------
    // EVENT ACCESS
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn event(
        &self,
    ) -> &EventMap {
        &self.event
    }

    #[inline(always)]
    pub fn event_mut(
        &mut self,
    ) -> &mut EventMap {
        &mut self.event
    }

    // -------------------------------------------------------------------------
    // AUTHENTIC HERITAGE
    //
    // Borrow only.
    // Ownership remains inside EventMap.
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn heritage(
        &self,
    ) -> &Heritage {
        self.event.heritage()
    }

    // -------------------------------------------------------------------------
    // DERIVED VALUES
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn activation(
        &self,
    ) -> u64 {
        self.event.activation()
    }

    #[inline(always)]
    pub fn dimension(
        &self,
    ) -> u64 {
        self.event.dimension()
    }

    #[inline(always)]
    pub fn structural_value(
        &self,
    ) -> u128 {
        self.event.structural_value()
    }

    #[inline(always)]
    pub fn retained_mass(
        &self,
    ) -> u128 {
        self.event.retained_mass()
    }

    #[inline(always)]
    pub fn retain(
        &self,
    ) -> Retain {
        Retain {
            mass: self.retained_mass(),
            from: self.event.dimension(),
        }
    }


    #[inline(always)]
    pub fn retain_bytes(
        &self,
    ) -> [u8; 24] {
        let retain = self.retain();
    
        let mut bytes = [0u8; 24];
    
        bytes[..16]
            .copy_from_slice(
                &retain.mass.to_le_bytes(),
            );
    
        bytes[16..24]
            .copy_from_slice(
                &retain.from.to_le_bytes(),
            );
    
        bytes
    }
    

    #[inline(always)]
    pub fn net_work(
        &self,
    ) -> u128 {
        self.event.net_work()
    }

    #[inline(always)]
    pub fn tau(
        &self,
    ) -> u128 {
        self.event.tau()
    }

    #[inline(always)]
    pub fn scale(
        &self,
    ) -> Scale {
        self.event.scale()
    }

    // -------------------------------------------------------------------------
    // OWNERSHIP TRANSFER
    //
    // Consume the direction and return the complete EventMap.
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn into_event(
        self,
    ) -> EventMap {
        self.event
    }

    // -------------------------------------------------------------------------
    // OWNERSHIP TRANSFER
    //
    // Consume the direction and return the authentic Heritage.
    // EventMap is consumed in the process.
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn into_heritage(
        self,
    ) -> Heritage {
        self.event.into_heritage()
    }
}




pub struct Depth {
    path: String,

    // Canonical object storage:
    // actual object occurrence → retained mass.
    objects: HashMap<usize, Retain>,

    // Protocol-state label:
    // position → identifier.
    positions: BTreeMap<usize, String>,
}

impl Depth {
    #[inline(always)]
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            objects: HashMap::new(),
            positions: BTreeMap::new(),
        }
    }

    // -------------------------------------------------------------------------
    // PATH
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline(always)]
    pub fn command(&self) -> &str {
        self.path
            .split('/')
            .next()
            .unwrap_or("")
    }

    #[inline(always)]
    pub fn commands(&self) -> impl Iterator<Item = &str> {
        self.path
            .split('/')
            .filter(|c| !c.is_empty())
    }

    #[inline(always)]
    pub fn next(&self) -> Option<&str> {
        self.commands().nth(1)
    }

    #[inline(always)]
    pub fn at(&self, depth: usize) -> Option<&str> {
        self.commands().nth(depth)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.commands().count()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    // -------------------------------------------------------------------------
    // OBJECT LOOKUP
    // -------------------------------------------------------------------------

    /// Lookup the actual retained object at a concrete position.
    #[inline(always)]
    pub fn position(
        &self,
        index: usize,
    ) -> Option<&Retain> {
        self.objects.get(&index)
    }

    /// Lookup the protocol identifier at a concrete position.
    #[inline(always)]
    pub fn identifier_at(
        &self,
        position: usize,
    ) -> Option<&str> {
        self.positions
            .get(&position)
            .map(String::as_str)
    }

    /// Return the next concrete object position.
    #[inline(always)]
    pub fn next_position(&self) -> usize {
        self.positions
            .keys()
            .next_back()
            .map_or(0, |position| position + 1)
    }

    // -------------------------------------------------------------------------
    // INSERT
    // -------------------------------------------------------------------------

    /// Insert one concrete object occurrence.
    ///
    /// `identifier` is only the protocol-state label.
    /// `position` identifies the actual occurrence.
    #[inline(always)]
    pub fn insert(
        &mut self,
        identifier: impl Into<String>,
        position: usize,
        value: Retain,
    ) {
        self.positions
            .insert(position, identifier.into());

        self.objects
            .insert(position, value);
    }

    // -------------------------------------------------------------------------
    // IDENTIFIER LOOKUP
    // -------------------------------------------------------------------------

    /// Find the retained object for the most recent occurrence
    /// carrying this identifier.
    #[inline(always)]
    pub fn object(
        &self,
        identifier: &str,
    ) -> Option<&Retain> {
        self.positions
            .iter()
            .rev()
            .find_map(|(position, id)| {
                if id == identifier {
                    self.objects.get(position)
                } else {
                    None
                }
            })
    }

    /// Iterate over all concrete objects in canonical position order.
    #[inline(always)]
    pub fn objects(
        &self,
    ) -> impl Iterator<Item = (usize, &str, &Retain)> {
        self.positions.iter().filter_map(
            |(position, identifier)| {
                self.objects
                    .get(position)
                    .map(|retain| {
                        (*position, identifier.as_str(), retain)
                    })
            },
        )
    }
}