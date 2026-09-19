use eframe::egui;
use quantom_value::{
    Perm,
    QuantPerm,
};

use qtm_graph::{
    Dashboard,
    DashboardBuilder,
    DashboardView,
    graph::{
        frame::{
            self,
            EventMap,
        },
        scale::Scale,
    },
};

struct PermDomainApp {
    dashboard: Dashboard,
    frame: EventMap,
}

impl PermDomainApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
    ) -> Self {
        let perm =
            Perm::from_u128(0);

        let mut quantperm =
            QuantPerm::new(perm);

        quantperm
            .set_initial_dimension_from_perm();

        let retain =
            quantperm.retain(
                0,
                perm.dimension() as u64,
            );


        // ---------------------------------------------------------------------
        // DEPTH
        // Depth is now the only realization boundary into the manifold.
        //
        // Even the initial zero-mass state must enter through Depth rather
        // than constructing Retain directly in main.rs.
        // ---------------------------------------------------------------------

        let mut depth =
            frame::Depth::new(
                "perm/genesis",
            );

         depth.insert(
            "genesis",
            0,
            retain,
        );

        // ---------------------------------------------------------------------
        // GRAPH IS THE ONLY TRANSITION GATE.
        //
        // main.rs never performs a direct Quantom-VALUE transition.
        // ---------------------------------------------------------------------

        let heritage =
            frame::Direction::transit(
                quantperm,
                &depth,
                None,
            );

        // ---------------------------------------------------------------------
        // EVENT FRAME.
        //
        // EventMap owns the authentic Heritage and runtime metadata.
        // ---------------------------------------------------------------------

        let frame =
            EventMap::now(
                heritage,
                Scale::Global,
            );

        // ---------------------------------------------------------------------
        // DASHBOARD.
        //
        // main.rs only selects the initial presentation mode.
        // Geometry and rendering remain inside graph modules.
        // ---------------------------------------------------------------------

        let mut dashboard =
            DashboardBuilder::new()
                .view(
                    DashboardView::DimensionNetwork,
                )
                .build();

        // ---------------------------------------------------------------------
        // LIVE OBSERVATION.
        //
        // Dashboard observes the EventMap; it does not own the event.
        // ---------------------------------------------------------------------

        dashboard.journey(
            &frame,
        );

        Self {
            dashboard,
            frame,
        }
    }
}

impl eframe::App for PermDomainApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        // =====================================================================
        // DASHBOARD HEADER
        // =====================================================================
        //
        // Keep the control area separate from the graph area so the selector
        // cannot overlap graph cells, labels, axes, or lattice content.
        //
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing =
                egui::vec2(8.0, 8.0);

            // -----------------------------------------------------------------
            // VIEW SELECTION
            // -----------------------------------------------------------------

            ui.horizontal(|ui| {
                ui.label("Dashboard:");

                egui::ComboBox::from_id_salt(
                    "dashboard_view",
                )
                .selected_text(
                    self.dashboard
                        .current_view()
                        .label(),
                )
                .width(240.0)
                .show_ui(
                    ui,
                    |ui| {
                        for view
                            in DashboardView::variants()
                        {
                            ui.selectable_value(
                                &mut self.dashboard.state.selected_view,
                                *view,
                                view.label(),
                            );
                        }
                    },
                );

                ui.separator();

                // -------------------------------------------------------------
                // SHOW ALL
                // -------------------------------------------------------------

                let mut show_all =
                    self.dashboard.state.show_all;

                if ui
                    .checkbox(
                        &mut show_all,
                        "Show All",
                    )
                    .changed()
                {
                    self.dashboard.state.show_all =
                        show_all;

                    if show_all {
                        self.dashboard
                            .select_view(
                                DashboardView::All,
                            );
                    } else {
                        self.dashboard
                            .select_view(
                                DashboardView::DimensionNetwork,
                            );
                    }
                }
            });

            // -----------------------------------------------------------------
            // HEADER / GRAPH SEPARATOR
            // -----------------------------------------------------------------

            ui.separator();

            // =================================================================
            // GRAPH REGION
            // =================================================================
            //
            // Give the graph its own layout region. The graph projection owns
            // all cells, axes, labels, lattice scaling and coordinate mapping.
            //
            ui.allocate_ui_with_layout(
                egui::vec2(
                    ui.available_width(),
                    ui.available_height(),
                ),
                egui::Layout::top_down(
                    egui::Align::Min,
                ),
                |ui| {
                    self.dashboard.render(
                        ui,
                        &self.frame,
                    );
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let options =
        eframe::NativeOptions {
            viewport:
                egui::ViewportBuilder::default()
                    .with_title(
                        "Quantom • PERM / QuantPerm Surface",
                    )
                    .with_inner_size([
                        1280.0,
                        820.0,
                    ])
                    .with_min_inner_size([
                        900.0,
                        640.0,
                    ]),
            ..Default::default()
        };

    eframe::run_native(
        "Quantom • PERM / QuantPerm Surface",
        options,
        Box::new(|cc| {
            Ok(Box::new(
                PermDomainApp::new(cc),
            ))
        }),
    )
}