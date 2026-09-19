
// =============================================================================
// QUANTPERM MAP 2D — Coordinate Mapping for Egui
// =============================================================================

use eframe::egui;
use quantom_value::Heritage;

#[derive(Clone, Copy, Debug)]
pub struct QuantPermMap2D {
    pub rect: egui::Rect,
}

impl QuantPermMap2D {
    #[inline(always)]
    pub fn new(rect: egui::Rect) -> Self {
        Self { rect }
    }

    #[inline(always)]
    pub fn x(&self, value: u128) -> f32 {
        let r = value as f64 / u128::MAX as f64;
        self.rect.left() + self.rect.width() * r as f32
    }

    #[inline(always)]
    pub fn x_dimension(&self, value: u64) -> f32 {
        let r = value as f64 / u64::MAX as f64;
        self.rect.left() + self.rect.width() * r as f32
    }

    #[inline(always)]
    pub fn y(&self, value: u64) -> f32 {
        let r = value as f64 / u64::MAX as f64;
        self.rect.bottom() - self.rect.height() * r as f32
    }

    #[inline(always)]
    pub fn y_network(&self, value: u128) -> f32 {
        let r = value as f64 / u128::MAX as f64;
        self.rect.bottom() - self.rect.height() * r as f32
    }

    #[inline(always)]
    pub fn position(&self, x: u128, y: u64) -> egui::Pos2 {
        egui::pos2(self.x(x), self.y(y))
    }

    #[inline(always)]
    pub fn dimension_network_position(&self, dimension: u64, network: u128) -> egui::Pos2 {
        egui::pos2(self.x_dimension(dimension), self.y_network(network))
    }
}

// =============================================================================
// GEOMETRIC PROJECTIONS — ANGLES & MIRRORS
// =============================================================================

#[inline(always)]
pub fn dimension_angle(heritage: &Heritage) -> f32 {
    ((heritage.state.dimension() as f64 / u64::MAX as f64) * heritage.transition.tau as f64) as f32
}

#[inline(always)]
pub fn structural_value_angle(heritage: &Heritage) -> f32 {
    ((heritage.state.structural_value() as f64 / u128::MAX as f64) * heritage.transition.tau as f64) as f32
}

#[inline(always)]
pub fn structural_value_mirror(heritage: &Heritage) -> f32 {
    let mirror = quantom_value::mirror_u128(&heritage.transition.mirror_bytes);
    ((heritage.state.structural_value() as f64 / u128::MAX as f64) * mirror as f64) as f32
}

#[inline(always)]
pub fn dimension_angle_mirror(heritage: &Heritage) -> f32 {
    let mirror = quantom_value::mirror_u128(&heritage.transition.mirror_bytes);
    ((heritage.state.dimension() as f64 / u64::MAX as f64) * mirror as f64) as f32
}

#[inline(always)]
pub fn dimension_network_point(heritage: &Heritage) -> (f32, f32) {
    let dimension = heritage.state.dimension();
    let network = heritage.transition.net_work;

    (
        (dimension as f64 / u64::MAX as f64) as f32,
        (network as f64 / u128::MAX as f64) as f32,
    )
}


// =============================================================================
// HELPERS
// =============================================================================

#[inline(always)]
fn allocate_painter(ui: &mut egui::Ui, height: f32) -> (egui::Rect, egui::Painter, egui::Visuals) {
    let desired = egui::vec2(ui.available_width(), height);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    (rect, painter, ui.visuals().clone())
}

#[inline(always)]
fn circle_params(rect: egui::Rect) -> (egui::Pos2, f32) {
    (rect.center(), rect.height().min(rect.width()) * 0.35)
}

#[inline(always)]
fn grid_stroke(ui: &egui::Ui) -> egui::Stroke {
    egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
}

#[inline(always)]
fn grid_line(painter: &egui::Painter, from: egui::Pos2, to: egui::Pos2, width: f32, color: egui::Color32) {
    painter.line_segment([from, to], egui::Stroke::new(width, color));
}

// =============================================================================
// WINDOW 1 — DIMENSION / NETWORK (X=Dimension, Y=Network)
// =============================================================================

#[inline(always)]
pub fn render_dimension_network(ui: &mut egui::Ui, heritage: &Heritage) {
    let rect = ui.available_rect_before_wrap();
    let map = QuantPermMap2D::new(rect);
    let painter = ui.painter_at(rect);
    let visuals = ui.visuals().clone();
    let axis = visuals.widgets.noninteractive.bg_stroke.color;

    painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);

    // ── Axes ──
    painter.line_segment([rect.left_bottom(), rect.right_bottom()], egui::Stroke::new(1.0, axis));
    painter.line_segment([rect.left_bottom(), rect.left_top()], egui::Stroke::new(1.0, axis));

    // ── Grid ──
    for i in 1..=4 {
        let frac = i as f32 / 4.0;
        let x = rect.left() + rect.width() * frac;
        let y = rect.bottom() - rect.height() * frac;
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], egui::Stroke::new(0.5, axis));
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], egui::Stroke::new(0.5, axis));
    }

    // ── Axis Labels ──
    let font = |s| egui::FontId::monospace(s);
    painter.text(rect.right_bottom() - egui::vec2(8.0, 8.0), egui::Align2::RIGHT_BOTTOM, "Dimension →", font(11.0), visuals.text_color());
    painter.text(rect.left_top() + egui::vec2(8.0, 8.0), egui::Align2::LEFT_TOP, "Network ↑", font(11.0), visuals.text_color());

    // ── QuantPerm Point ──
    let pos = map.dimension_network_position(heritage.state.dimension(), heritage.transition.net_work);
    painter.circle_filled(pos, 5.0, visuals.selection.bg_fill);
    painter.circle_stroke(pos, 9.0, egui::Stroke::new(1.0, visuals.selection.stroke.color));

    // ── Point Info ──
    let info = [
        ("QuantPerm", 11.0, visuals.text_color()),
        (&format!("dimension={}", heritage.state.dimension()), 10.0, visuals.weak_text_color()),
        (&format!("network={}", heritage.transition.net_work), 10.0, visuals.weak_text_color()),
        (&format!("activations={}", heritage.state.activations()), 10.0, visuals.weak_text_color()),
    ];
    for (i, (text, size, color)) in info.iter().enumerate() {
        painter.text(
            pos + egui::vec2(10.0, 8.0 + i as f32 * 16.0),
            egui::Align2::LEFT_TOP,
            *text,
            font(*size),
            *color,
        );
    }

    // ── Title ──
    painter.text(
        rect.left_top() + egui::vec2(12.0, 28.0),
        egui::Align2::LEFT_TOP,
        "QuantPerm • Dimension / Network",
        font(12.0),
        visuals.selection.stroke.color,
    );
}

// =============================================================================
// SHARED PROTOCOL LATTICE
//
// The protocol surface is conceptually:
//
//     X = 0 ..= u128::MAX
//     Y = 0 ..= u128::MAX
//
// We never allocate those astronomical protocol cells.
//
// The renderer exposes a finite screen lattice. Every visible screen cell is
// a window into a rectangular range of the authoritative u128 × u128 domain:
//
//     cell = f(X_range, Y_range)
//
// Therefore:
//
//     16 × 16 = 256 visible cells
//     64 × 64 = 4096 visible cells
//
// regardless of the size of the underlying protocol domain.
//
// Zoom changes the protocol-space range represented by the window. It does not
// change, truncate, or allocate the underlying u128 domain.
// =============================================================================

const LATTICE_MIN_CELLS: u32 = 4;
const LATTICE_MAX_CELLS: u32 = 64;
const LATTICE_DEFAULT_CELLS: u32 = 16;

const LATTICE_MIN_ZOOM: f64 = 1.0;
const LATTICE_MAX_ZOOM: f64 = 1_048_576.0;
const LATTICE_ZOOM_STEP: f64 = 1.25;

// =============================================================================
// LATTICE VIEWPORT
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct LatticeViewport {
    // 1.0 = complete protocol domain.
    // Higher values zoom into a smaller protocol-space window.
    zoom: f64,

    // Normalized protocol-space center.
    // 0.0 = minimum, 1.0 = maximum.
    center_x: f64,
    center_y: f64,
}

impl Default for LatticeViewport {
    fn default() -> Self {
        Self {
            zoom: LATTICE_MIN_ZOOM,
            center_x: 0.5,
            center_y: 0.5,
        }
    }
}

impl LatticeViewport {
    #[inline(always)]
    fn zoomed(mut self, factor: f64) -> Self {
        self.zoom = (self.zoom * factor)
            .clamp(LATTICE_MIN_ZOOM, LATTICE_MAX_ZOOM);
        self
    }

    #[inline(always)]
    fn cells(&self, rect: egui::Rect) -> u32 {
        let base = rect.width().min(rect.height()) / 24.0;

        (base.round() as u32).clamp(
            LATTICE_MIN_CELLS,
            LATTICE_MAX_CELLS,
        )
    }

    // -------------------------------------------------------------------------
    // Visible normalized range.
    //
    // At zoom=1 the entire [0,1] × [0,1] protocol domain is visible.
    // At zoom=1024 only 1/1024 of the domain is visible in each axis.
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn span(&self) -> f64 {
        (1.0 / self.zoom).clamp(
            1.0 / LATTICE_MAX_ZOOM,
            1.0,
        )
    }

    #[inline(always)]
    fn x_range(&self) -> (f64, f64) {
        Self::axis_range(self.center_x, self.span())
    }

    #[inline(always)]
    fn y_range(&self) -> (f64, f64) {
        Self::axis_range(self.center_y, self.span())
    }

    #[inline(always)]
    fn axis_range(center: f64, span: f64) -> (f64, f64) {
        let half = span * 0.5;
        let mut min = center.clamp(0.0, 1.0) - half;
        let mut max = center.clamp(0.0, 1.0) + half;

        if min < 0.0 {
            max = (max - min).min(1.0);
            min = 0.0;
        }

        if max > 1.0 {
            min = (min - (max - 1.0)).max(0.0);
            max = 1.0;
        }

        (min, max)
    }
}

// =============================================================================
// PROTOCOL LATTICE
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct ProtocolLattice {
    rect: egui::Rect,
    cells: u32,
    viewport: LatticeViewport,
}

impl ProtocolLattice {
    #[inline(always)]
    fn new(
        rect: egui::Rect,
        viewport: LatticeViewport,
    ) -> Self {
        Self {
            rect,
            cells: viewport
                .cells(rect)
                .clamp(
                    LATTICE_MIN_CELLS,
                    LATTICE_MAX_CELLS,
                ),
            viewport,
        }
    }

    // -------------------------------------------------------------------------
    // PROTOCOL NORMALIZATION
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn normalize(value: u128) -> f64 {
        value as f64 / u128::MAX as f64
    }

    // -------------------------------------------------------------------------
    // VISIBLE PROTOCOL RANGE
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn x_range(&self) -> (f64, f64) {
        self.viewport.x_range()
    }

    #[inline(always)]
    fn y_range(&self) -> (f64, f64) {
        self.viewport.y_range()
    }

    // -------------------------------------------------------------------------
    // NORMALIZED PROTOCOL VALUE -> SCREEN
    //
    // The conversion to f32 happens only at the final egui boundary.
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn screen(
        &self,
        x: f64,
        y: f64,
    ) -> egui::Pos2 {
        let (x0, x1) = self.x_range();
        let (y0, y1) = self.y_range();

        let sx = if x1 > x0 {
            ((x - x0) / (x1 - x0)).clamp(0.0, 1.0)
        } else {
            0.5
        };

        let sy = if y1 > y0 {
            ((y - y0) / (y1 - y0)).clamp(0.0, 1.0)
        } else {
            0.5
        };

        egui::pos2(
            self.rect.left()
                + self.rect.width() * sx as f32,
            self.rect.bottom()
                - self.rect.height() * sy as f32,
        )
    }

    // -------------------------------------------------------------------------
    // PROTOCOL VALUE -> SCREEN
    //
    // Authoritative values remain u128 until the final rendering conversion.
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn point(
        &self,
        x: u128,
        y: u128,
    ) -> egui::Pos2 {
        self.screen(
            Self::normalize(x),
            Self::normalize(y),
        )
    }

    // -------------------------------------------------------------------------
    // CELL RANGE
    //
    // Returns the authoritative protocol-space range represented by a visible
    // screen cell.
    //
    // The returned ranges are inclusive logical boundaries:
    //
    //     ((x_min, x_max), (y_min, y_max))
    //
    // No protocol-space cells are allocated.
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn cell_range(
        &self,
        column: u32,
        row: u32,
    ) -> ((u128, u128), (u128, u128)) {
        let n = self.cells as f64;

        let (x0, x1) = self.x_range();
        let (y0, y1) = self.y_range();

        let cx0 = x0 + (x1 - x0) * column as f64 / n;
        let cx1 = if column + 1 >= self.cells {
            x1
        } else {
            x0 + (x1 - x0) * (column + 1) as f64 / n
        };

        let cy0 = y0 + (y1 - y0) * row as f64 / n;
        let cy1 = if row + 1 >= self.cells {
            y1
        } else {
            y0 + (y1 - y0) * (row + 1) as f64 / n
        };

        (
            (
                Self::to_u128(cx0),
                Self::to_u128(cx1),
            ),
            (
                Self::to_u128(cy0),
                Self::to_u128(cy1),
            ),
        )
    }

    #[inline(always)]
    fn to_u128(value: f64) -> u128 {
        if value <= 0.0 {
            return 0;
        }

        if value >= 1.0 {
            return u128::MAX;
        }

        (value * u128::MAX as f64) as u128
    }

    // -------------------------------------------------------------------------
    // CELL RECT
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn cell_rect(
        &self,
        column: u32,
        row: u32,
    ) -> egui::Rect {
        let n = self.cells as f32;

        let width = self.rect.width() / n;
        let height = self.rect.height() / n;

        egui::Rect::from_min_max(
            egui::pos2(
                self.rect.left()
                    + column as f32 * width,
                self.rect.bottom()
                    - (row + 1) as f32 * height,
            ),
            egui::pos2(
                self.rect.left()
                    + (column + 1) as f32 * width,
                self.rect.bottom()
                    - row as f32 * height,
            ),
        )
    }

    // -------------------------------------------------------------------------
    // CELL CENTER
    // -------------------------------------------------------------------------

    #[inline(always)]
    fn cell_center(
        &self,
        column: u32,
        row: u32,
    ) -> egui::Pos2 {
        self.cell_rect(column, row).center()
    }
}

// =============================================================================
// SHARED LATTICE WINDOW
//
// Every shared 2D window is an organized row/column lattice.
//
// The visible lattice is finite:
//
//     cells × cells
//
// The conceptual protocol lattice is not finite:
//
//     (u128::MAX + 1) × (u128::MAX + 1)
//
// which is approximately 2^256 possible coordinate pairs.
//
// We therefore treat each rendered cell as a micro-scale bucket over a
// protocol-space range. Zoom narrows the range represented by those same
// visible cells.
//
// The renderer never allocates the protocol cells themselves.
// =============================================================================

#[inline(always)]
fn render_lattice_window(
    ui: &mut egui::Ui,
    title: &str,
    point: Option<(u128, u128)>,
    value_text: Option<String>,
) {
    let desired = egui::vec2(
        ui.available_width(),
        300.0,
    );

    let (rect, _) = ui.allocate_exact_size(
        desired,
        egui::Sense::hover(),
    );

    let painter = ui.painter_at(rect);
    let visuals = ui.visuals().clone();

    let id = ui.make_persistent_id((
        "quantperm_lattice",
        title,
    ));

    let mut viewport = ui.ctx().data(|data| {
        data.get_temp::<LatticeViewport>(id)
            .unwrap_or_default()
    });

    // -------------------------------------------------------------------------
    // POINTER-LOCAL ZOOM
    //
    // The lattice zooms around the current pointer location. This makes zoom
    // useful as a micro-scale inspection tool without changing protocol values.
    // -------------------------------------------------------------------------

    let pointer = ui.input(|i| i.pointer.hover_pos());
    let hovered = pointer
        .map(|p| rect.contains(p))
        .unwrap_or(false);

    if hovered {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);

        if scroll != 0.0 {
            let factor = if scroll > 0.0 {
                LATTICE_ZOOM_STEP
            } else {
                1.0 / LATTICE_ZOOM_STEP
            };

            viewport = viewport.zoomed(factor);

            if let Some(pointer) = pointer {
                let nx = if rect.width() > 0.0 {
                    ((pointer.x - rect.left())
                        / rect.width()) as f64
                } else {
                    0.5
                };

                let ny = if rect.height() > 0.0 {
                    ((rect.bottom() - pointer.y)
                        / rect.height()) as f64
                } else {
                    0.5
                };

                let (x0, x1) = viewport.x_range();
                let (y0, y1) = viewport.y_range();

                viewport.center_x =
                    (x0 + (x1 - x0) * nx)
                        .clamp(0.0, 1.0);

                viewport.center_y =
                    (y0 + (y1 - y0) * ny)
                        .clamp(0.0, 1.0);
            }

            ui.ctx().data_mut(|data| {
                data.insert_temp(id, viewport);
            });
        }
    }

    // -------------------------------------------------------------------------
    // BACKGROUND
    // -------------------------------------------------------------------------

    painter.rect_filled(
        rect,
        0.0,
        visuals.extreme_bg_color,
    );

    let lattice = ProtocolLattice::new(
        rect.shrink(18.0),
        viewport,
    );

    let grid = visuals
        .widgets
        .noninteractive
        .bg_stroke
        .color;

    // -------------------------------------------------------------------------
    // LATTICE
    //
    // Only visible screen cells are rendered.
    //
    // The number of protocol-space cells is never instantiated.
    // -------------------------------------------------------------------------

    for row in 0..lattice.cells {
        for column in 0..lattice.cells {
            let cell = lattice.cell_rect(
                column,
                row,
            );

            painter.rect_stroke(
                cell,
                0.0,
                egui::Stroke::new(
                    0.5,
                    grid,
                ),
                egui::StrokeKind::Inside,
            );
        }
    }

    // -------------------------------------------------------------------------
    // AXES
    //
    // Bottom-left is protocol (0,0).
    // Top-right is protocol (u128::MAX,u128::MAX).
    // At high zoom these labels describe the viewport boundary instead.
    // -------------------------------------------------------------------------

    painter.line_segment(
        [
            lattice.rect.left_bottom(),
            lattice.rect.right_bottom(),
        ],
        egui::Stroke::new(
            1.0,
            visuals.text_color(),
        ),
    );

    painter.line_segment(
        [
            lattice.rect.left_bottom(),
            lattice.rect.left_top(),
        ],
        egui::Stroke::new(
            1.0,
            visuals.text_color(),
        ),
    );

    // -------------------------------------------------------------------------
    // EVENT / PROJECTION POINT
    // -------------------------------------------------------------------------

    if let Some((x, y)) = point {
        let position = lattice.point(
            x,
            y,
        );

        painter.circle_filled(
            position,
            6.0,
            visuals.selection.bg_fill,
        );

        painter.circle_stroke(
            position,
            11.0,
            egui::Stroke::new(
                1.0,
                visuals.selection.stroke.color,
            ),
        );

        // Highlight the containing micro-cell.
        let px = ProtocolLattice::normalize(x);
        let py = ProtocolLattice::normalize(y);

        let (x0, x1) = lattice.x_range();
        let (y0, y1) = lattice.y_range();

        if px >= x0 && px <= x1
            && py >= y0 && py <= y1
        {
            let column = (((px - x0)
                / (x1 - x0).max(f64::MIN_POSITIVE))
                * lattice.cells as f64)
                .floor() as u32;

            let row = (((py - y0)
                / (y1 - y0).max(f64::MIN_POSITIVE))
                * lattice.cells as f64)
                .floor() as u32;

            let column = column.min(
                lattice.cells.saturating_sub(1),
            );

            let row = row.min(
                lattice.cells.saturating_sub(1),
            );

            painter.rect_stroke(
                lattice.cell_rect(
                    column,
                    row,
                ),
                0.0,
                egui::Stroke::new(
                    1.5,
                    visuals.selection.stroke.color,
                ),
                egui::StrokeKind::Inside,
            );
        }
    }

    // -------------------------------------------------------------------------
    // TITLE
    // -------------------------------------------------------------------------

    painter.text(
        rect.left_top()
            + egui::vec2(
                12.0,
                8.0,
            ),
        egui::Align2::LEFT_TOP,
        title,
        egui::FontId::monospace(12.0),
        visuals.text_color(),
    );

    // -------------------------------------------------------------------------
    // DOMAIN LABELS
    // -------------------------------------------------------------------------

    painter.text(
        rect.right_bottom()
            - egui::vec2(
                8.0,
                8.0,
            ),
        egui::Align2::RIGHT_BOTTOM,
        "X = u128::MAX →",
        egui::FontId::monospace(9.0),
        visuals.weak_text_color(),
    );

    painter.text(
        rect.left_top()
            + egui::vec2(
                8.0,
                28.0,
            ),
        egui::Align2::LEFT_TOP,
        "Y = u128::MAX ↑",
        egui::FontId::monospace(9.0),
        visuals.weak_text_color(),
    );

    // -------------------------------------------------------------------------
    // SCALE INFORMATION
    // -------------------------------------------------------------------------

    let cell_count =
        lattice.cells as u64
            * lattice.cells as u64;

    let (x0, x1) = lattice.x_range();
    let (y0, y1) = lattice.y_range();

    painter.text(
        rect.right_top()
            + egui::vec2(
                -12.0,
                8.0,
            ),
        egui::Align2::RIGHT_TOP,
        format!(
            "{} × {} cells • {} visible • zoom {:.2}×",
            lattice.cells,
            lattice.cells,
            cell_count,
            viewport.zoom,
        ),
        egui::FontId::monospace(9.0),
        visuals.weak_text_color(),
    );

    painter.text(
        rect.left_bottom()
            + egui::vec2(
                8.0,
                -8.0,
            ),
        egui::Align2::LEFT_BOTTOM,
        format!(
            "X [{:.6e}, {:.6e}] • Y [{:.6e}, {:.6e}]",
            x0,
            x1,
            y0,
            y1,
        ),
        egui::FontId::monospace(8.0),
        visuals.weak_text_color(),
    );

    if let Some(value) = value_text {
        painter.text(
            rect.center_top()
                + egui::vec2(
                    0.0,
                    8.0,
                ),
            egui::Align2::CENTER_TOP,
            value,
            egui::FontId::monospace(10.0),
            visuals.selection.stroke.color,
        );
    }
}

// =============================================================================
// SHARED ANGLE WINDOW
//
// The angle remains the projection being displayed.
//
// The lattice is the spatial substrate underneath it. The angle is mapped
// deterministically into the authoritative u128 coordinate domain.
//
// A scalar projection therefore becomes a coordinate:
//
//     angle -> X
//     angle -> Y
//
// and consequently participates in the same:
//
//     Cell = f(X, Y)
//
// model as every other 2D projection.
// =============================================================================

#[inline(always)]
fn render_angle_window(
    ui: &mut egui::Ui,
    title: &str,
    angle: f32,
) {
    let normalized = (
        angle as f64
            / std::f64::consts::TAU
    )
    .rem_euclid(1.0);

    let coordinate =
        (normalized
            * u128::MAX as f64)
            as u128;

    render_lattice_window(
        ui,
        title,
        Some((
            coordinate,
            coordinate,
        )),
        Some(
            format!(
                "angle={:.6} rad",
                angle,
            ),
        ),
    );
}

// =============================================================================
// SHARED SCALAR WINDOW
//
// A scalar remains a scalar projection, but it is placed into the same
// protocol lattice.
//
// The scalar's normalized position is represented on both axes so the cell
// identity remains:
//
//     Cell = f(X, Y)
//
// This keeps all shared windows geometrically compatible without changing
// their existing public APIs.
// =============================================================================

#[inline(always)]
fn render_scalar_window(
    ui: &mut egui::Ui,
    title: &str,
    value: f32,
) {
    let normalized =
        value
            .clamp(0.0, 1.0)
            as f64;

    let coordinate =
        (normalized
            * u128::MAX as f64)
            as u128;

    render_lattice_window(
        ui,
        title,
        Some((
            coordinate,
            coordinate,
        )),
        Some(
            format!(
                "value={:.6}",
                value,
            ),
        ),
    );
}

// =============================================================================
// WINDOWS 2–6 — PUBLIC API
// =============================================================================

#[inline(always)]
pub fn render_dimension_angle(
    ui: &mut egui::Ui,
    heritage: &Heritage,
) {
    render_angle_window(
        ui,
        "Dimension Angle",
        dimension_angle(heritage),
    );
}

#[inline(always)]
pub fn render_structural_value_angle(
    ui: &mut egui::Ui,
    heritage: &Heritage,
) {
    render_angle_window(
        ui,
        "Structural Value Angle",
        structural_value_angle(heritage),
    );
}

#[inline(always)]
pub fn render_structural_value_mirror(
    ui: &mut egui::Ui,
    heritage: &Heritage,
) {
    render_scalar_window(
        ui,
        "Structural Value Mirror",
        structural_value_mirror(heritage),
    );
}

#[inline(always)]
pub fn render_dimension_angle_mirror(
    ui: &mut egui::Ui,
    heritage: &Heritage,
) {
    render_scalar_window(
        ui,
        "Dimension Angle Mirror",
        dimension_angle_mirror(heritage),
    );
}

#[inline(always)]
pub fn render_dimension_network_point(
    ui: &mut egui::Ui,
    heritage: &Heritage,
) {
    let dimension =
        heritage.state.dimension();

    let network =
        heritage.transition.net_work;

    render_lattice_window(
        ui,
        "Dimension / Network Point",
        Some((
            dimension as u128,
            network,
        )),
        Some(
            format!(
                "dimension={} • network={}",
                dimension,
                network,
            ),
        ),
    );
}
