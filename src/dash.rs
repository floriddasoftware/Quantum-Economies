use eframe::egui;
use std::collections::HashMap;

use crate::graph::{
    frame::EventMap,
    qpdimensions2d,
};

// =============================================================================
// DASHBOARD VIEW
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardView {
    DimensionNetwork,
    DimensionAngle,
    StructuralValueAngle,
    StructuralValueMirror,
    DimensionAngleMirror,
    DimensionNetworkPoint,
    All,
}

impl DashboardView {
    // -------------------------------------------------------------------------
    // Selectable individual windows.
    //
    // `All` is intentionally excluded because it is an aggregate mode rather
    // than an individual rendering window.
    // -------------------------------------------------------------------------

    #[inline(always)]
    pub fn variants() -> &'static [Self] {
        use DashboardView::*;

        &[
            DimensionNetwork,
            DimensionAngle,
            StructuralValueAngle,
            StructuralValueMirror,
            DimensionAngleMirror,
            DimensionNetworkPoint,
        ]
    }

    #[inline(always)]
    pub fn label(&self) -> &'static str {
        use DashboardView::*;

        match self {
            DimensionNetwork =>
                "Dimension / Network",

            DimensionAngle =>
                "Dimension Angle",

            StructuralValueAngle =>
                "Structural Value Angle",

            StructuralValueMirror =>
                "Structural Value Mirror",

            DimensionAngleMirror =>
                "Dimension Angle Mirror",

            DimensionNetworkPoint =>
                "Dimension / Network Point",

            All =>
                "All Views",
        }
    }
}

// =============================================================================
// DASHBOARD STATE
// =============================================================================

#[derive(Debug)]
pub struct DashState {
    // Historical live states indexed by activation.
    pub states: HashMap<u64, ViewState>,

    // Current rendering selection.
    pub selected_view: DashboardView,

    // Aggregate rendering mode.
    pub show_all: bool,
}

// =============================================================================
// LIVE VIEW STATE
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct ViewState {
    pub structural_value: u128,
    pub net_work: u128,
    pub tau: u128,
    pub activations: u64,
}

// =============================================================================
// DASHBOARD
//
// Dashboard is the observer / presentation fan-out layer.
//
// OWNS:
//
//     - live view state
//     - selected rendering window
//     - caller-controlled view selection
//
// DOES NOT OWN:
//
//     - protocol geometry
//     - coordinate derivation
//     - rendering mathematics
//     - Quantom-VALUE transitions
//     - EventMap
//     - Heritage
//
// EventMap remains the graph-owned boundary around the authentic event.
//
// All geometry is delegated to graph/qpdimensions2d.rs.
// =============================================================================

pub struct Dashboard {
    pub state: DashState,
}

impl Dashboard {
    // =========================================================================
    // CONSTRUCTION
    // =========================================================================

    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: DashState {
                states: HashMap::new(),
                selected_view: DashboardView::DimensionNetwork,
                show_all: false,
            },
        }
    }

    // =========================================================================
    // VIEW SELECTION
    // =========================================================================

    #[inline(always)]
    pub fn select_view(
        &mut self,
        view: DashboardView,
    ) {
        self.state.selected_view = view;
        self.state.show_all =
            view == DashboardView::All;
    }

    #[inline(always)]
    pub fn toggle_show_all(
        &mut self,
    ) {
        self.state.show_all =
            !self.state.show_all;

        if self.state.show_all {
            self.state.selected_view =
                DashboardView::All;
        } else if self.state.selected_view ==
            DashboardView::All
        {
            self.state.selected_view =
                DashboardView::DimensionNetwork;
        }
    }

    #[inline(always)]
    pub fn current_view(
        &self,
    ) -> DashboardView {
        self.state.selected_view
    }

    // =========================================================================
    // LIVE EVENT OBSERVER
    //
    // EventMap is the graph-owned boundary around the authentic event.
    //
    // No clone.
    // No ownership transfer.
    // No direct protocol mutation.
    // =========================================================================

    #[inline(always)]
    pub fn journey(
        &mut self,
        event: &EventMap,
    ) {
        let activations =
            event.activations();

        let structural_value =
            event.structural_value();

        let net_work =
            event.net_work();

        let tau =
            event.tau();

        let entry =
            self.state
                .states
                .entry(activations)
                .or_insert(
                    ViewState {
                        structural_value,
                        net_work,
                        tau,
                        activations,
                    },
                );

        // ---------------------------------------------------------------------
        // Deterministic live-state update.
        // ---------------------------------------------------------------------

        entry.structural_value =
            structural_value;

        entry.net_work =
            net_work;

        entry.tau =
            tau;

        entry.activations =
            activations;
    }

    // =========================================================================
    // SELECTED VIEW
    //
    // Main dashboard rendering entrypoint.
    //
    // The dashboard does not calculate geometry. It only chooses which graph
    // projection receives the authentic Heritage reference.
    // =========================================================================

    #[inline(always)]
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
    ) {
        self.render_view(
            ui,
            event,
            self.state.selected_view,
        );
    }

    // =========================================================================
    // EXPLICIT VIEW
    //
    // Caller can bypass dashboard state and request a specific window.
    // =========================================================================

    #[inline(always)]
    pub fn render_view(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
        view: DashboardView,
    ) {
        let heritage =
            event.heritage();

        match view {
            DashboardView::DimensionNetwork => {
                qpdimensions2d::render_dimension_network(
                    ui,
                    heritage,
                );
            }

            DashboardView::DimensionAngle => {
                qpdimensions2d::render_dimension_angle(
                    ui,
                    heritage,
                );
            }

            DashboardView::StructuralValueAngle => {
                qpdimensions2d::render_structural_value_angle(
                    ui,
                    heritage,
                );
            }

            DashboardView::StructuralValueMirror => {
                qpdimensions2d::render_structural_value_mirror(
                    ui,
                    heritage,
                );
            }

            DashboardView::DimensionAngleMirror => {
                qpdimensions2d::render_dimension_angle_mirror(
                    ui,
                    heritage,
                );
            }

            DashboardView::DimensionNetworkPoint => {
                qpdimensions2d::render_dimension_network_point(
                    ui,
                    heritage,
                );
            }

            DashboardView::All => {
                self.render_all(
                    ui,
                    event,
                );
            }
        }
    }

    // =========================================================================
    // ALL WINDOWS
    //
    // Each projection remains an independent window.
    //
    // The underlying lattice belongs to the graph projection, not Dashboard.
    // =========================================================================

    #[inline(always)]
    pub fn render_all(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
    ) {
        use DashboardView::*;

        let views = [
            DimensionNetwork,
            DimensionAngle,
            StructuralValueAngle,
            StructuralValueMirror,
            DimensionAngleMirror,
            DimensionNetworkPoint,
        ];

        for view in views {
            ui.collapsing(
                view.label(),
                |ui| {
                    self.render_view(
                        ui,
                        event,
                        view,
                    );
                },
            );
        }
    }

    // =========================================================================
    // RENDER WITH CONTROLS
    //
    // User/caller manually selects the active rendering window.
    // =========================================================================

    #[inline(always)]
    pub fn render_with_controls(
        &mut self,
        ui: &mut egui::Ui,
        event: &EventMap,
    ) {
        ui.horizontal(|ui| {
            ui.label("View:");

            egui::ComboBox::from_id_salt(
                "dashboard_view_selector",
            )
            .selected_text(
                self.state
                    .selected_view
                    .label(),
            )
            .show_ui(
                ui,
                |ui| {
                    for view
                        in DashboardView::variants()
                    {
                        ui.selectable_value(
                            &mut self.state.selected_view,
                            *view,
                            view.label(),
                        );
                    }
                },
            );

            ui.separator();

            if ui
                .checkbox(
                    &mut self.state.show_all,
                    "Show All",
                )
                .changed()
            {
                self.state.selected_view =
                    if self.state.show_all {
                        DashboardView::All
                    } else {
                        DashboardView::DimensionNetwork
                    };
            }
        });

        ui.separator();

        self.render(
            ui,
            event,
        );
    }
}

// =============================================================================
// DASHBOARD VIEW BUILDER
// =============================================================================

#[derive(Debug, Default)]
pub struct DashboardBuilder {
    view: Option<DashboardView>,
    show_all: bool,
}

impl DashboardBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn view(
        mut self,
        view: DashboardView,
    ) -> Self {
        self.view = Some(view);
        self
    }

    #[inline(always)]
    pub fn show_all(
        mut self,
    ) -> Self {
        self.show_all = true;
        self
    }

    #[inline(always)]
    pub fn build(
        self,
    ) -> Dashboard {
        let mut dashboard =
            Dashboard::new();

        if let Some(view) =
            self.view
        {
            dashboard.select_view(view);
        }

        if self.show_all {
            dashboard.state.show_all = true;
            dashboard.state.selected_view =
                DashboardView::All;
        }

        dashboard
    }
}

// =============================================================================
// EXTENSION TRAIT
//
// Caller-owned rendering.
//
// Dashboard does not impose a particular UI layout.
// =============================================================================

pub trait DashboardExt {
    fn render_selected(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
        view: DashboardView,
    );

    fn render_side_by_side(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
        views: &[DashboardView],
    );
}

impl DashboardExt for Dashboard {
    #[inline(always)]
    fn render_selected(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
        view: DashboardView,
    ) {
        self.render_view(
            ui,
            event,
            view,
        );
    }

    #[inline(always)]
    fn render_side_by_side(
        &self,
        ui: &mut egui::Ui,
        event: &EventMap,
        views: &[DashboardView],
    ) {
        if views.is_empty() {
            return;
        }

        let count =
            views.len();

        ui.columns(
            count,
            |columns| {
                for (
                    column,
                    view,
                ) in columns
                    .iter_mut()
                    .zip(views.iter())
                {
                    self.render_view(
                        column,
                        event,
                        *view,
                    );
                }
            },
        );
    }
}