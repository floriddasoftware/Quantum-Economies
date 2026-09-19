use eframe::egui;
use quantom_value::{Heritage, Perm, QuantPerm};

pub(crate) const U128_MAX_F64: f64 = u128::MAX as f64;
pub(crate) const TAU: f32 = std::f32::consts::TAU;
pub(crate) const DOMAIN_SIZE: usize = 2048;
// -----------------------------------------------------------------------------
// QTM 2D MAP SCALE
// -----------------------------------------------------------------------------
#[derive(Debug,Clone,Copy)] pub struct MapScale2D{rect:egui::Rect}
impl MapScale2D{
    #[inline(always)] pub fn new(rect:egui::Rect)->Self{Self{rect}}
    #[inline(always)] pub fn rect(&self)->egui::Rect{self.rect}
    #[inline(always)] pub fn x(&self,value:u128)->f32{let r=value as f64/U128_MAX_F64; self.rect.left() + (self.rect.width() as f64 * r) as f32}
    #[inline(always)] pub fn y(&self,value:u128)->f32{let r=value as f64/U128_MAX_F64; self.rect.bottom() - (self.rect.height() as f64 * r) as f32}
    #[inline(always)] pub fn point(&self,x:u128,y:u128)->egui::Pos2{egui::pos2(self.x(x), self.y(y))}
    #[inline(always)] pub fn inset(rect:egui::Rect,margin:f32)->Self{Self::new(rect.shrink(margin))}
    #[inline(always)] pub fn value_x(&self,x:f32)->u128{let w=self.rect.width(); if w<=0.0 { return 0; } let r=((x-self.rect.left())/w).clamp(0.0,1.0); (r as f64 * U128_MAX_F64) as u128}
    #[inline(always)] pub fn value_y(&self,y:f32)->u128{let h=self.rect.height(); if h<=0.0 { return 0; } let r=((self.rect.bottom()-y)/h).clamp(0.0,1.0); (r as f64 * U128_MAX_F64) as u128}
    #[inline(always)] pub fn value(&self,pos:egui::Pos2)->(u128,u128){(self.value_x(pos.x), self.value_y(pos.y))}
    #[inline(always)] pub fn normalize(v:u128)->f32{(v as f64 / U128_MAX_F64) as f32}
    #[inline(always)] pub fn denormalize(v:f32)->u128{(v.clamp(0.0,1.0) as f64 * U128_MAX_F64) as u128}
    #[inline(always)] pub fn normalized_point(x:u128,y:u128)->egui::Vec2{egui::vec2(Self::normalize(x), 1.0 - Self::normalize(y))}
}

// -----------------------------------------------------------------------------
// GENERIC MAP POINT
// -----------------------------------------------------------------------------
#[derive(Debug,Clone,Copy)] pub struct MapPoint{pub x:u128,pub y:u128}
impl MapPoint{
    #[inline(always)] pub const fn new(x:u128,y:u128)->Self{Self{x,y}}
    #[inline(always)] pub fn screen(self,scale:&MapScale2D)->egui::Pos2{scale.point(self.x,self.y)}
}

// -----------------------------------------------------------------------------
// 2D DOMAIN APPLICATION
// -----------------------------------------------------------------------------
pub struct PermDomainApp{ pub perm:Perm, pub heritage:Heritage, pub selected:Option<usize> }

impl PermDomainApp{
    pub fn new(_cc:&eframe::CreationContext<'_>)->Self{
        let perm = Perm::from_u128(0);
        let mut quantperm = QuantPerm::new(perm);
        quantperm.set_initial_dimension_from_perm();
        let retain = quantperm.retain(0, perm.dimension() as u64);
        let heritage = quantperm.transition(&retain, None);
        Self{perm, heritage, selected:None}
    }

    pub fn render_map(ui:&mut egui::Ui, perm:&Perm, heritage:&Heritage, selected:&mut Option<usize>){
        let rect = ui.available_rect_before_wrap();
        let map = SphereMap::new(rect);
        Self::handle_pointer(ui, &map, selected); // mutable UI first
        let painter = ui.painter_at(rect);
        let visuals = ui.visuals().clone();
        painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);
        Self::draw_perimeter(&painter, &map, &visuals);
        Self::draw_perm_domain(&painter, &map, perm, *selected, &visuals);
        Self::draw_quantperm(&painter, &map, heritage, &visuals);
        Self::draw_labels(&painter, &map, heritage, &visuals);
    }

    pub fn draw_perimeter(painter:&egui::Painter, map:&SphereMap, visuals:&egui::Visuals){
        painter.circle_stroke(map.center, map.radius, egui::Stroke::new(1.5, visuals.text_color()));
        let mut points = Vec::with_capacity(128);
        for i in 0..128 { let angle = i as f32 * TAU / 128.0; points.push(egui::pos2(map.center.x + angle.cos()*map.radius, map.center.y + angle.sin()*map.radius*0.30)); }
        points.push(points[0]);
        painter.add(egui::Shape::line(points, egui::Stroke::new(1.5, visuals.selection.stroke.color)));
        painter.line_segment([egui::pos2(map.center.x, map.center.y - map.radius), egui::pos2(map.center.x, map.center.y + map.radius)], egui::Stroke::new(0.75, visuals.widgets.noninteractive.bg_stroke.color));
        painter.line_segment([egui::pos2(map.center.x - map.radius, map.center.y), egui::pos2(map.center.x + map.radius, map.center.y)], egui::Stroke::new(0.75, visuals.widgets.noninteractive.bg_stroke.color));
        painter.circle_filled(map.center, 3.0, visuals.text_color());
    }

    pub fn draw_perm_domain(painter:&egui::Painter, map:&SphereMap, perm:&Perm, selected:Option<usize>, visuals:&egui::Visuals){
        let base_dimension = perm.dimension();
        for index in 0..DOMAIN_SIZE {
            let angle = index as f32 * TAU / DOMAIN_SIZE as f32;
            let position = map.perm_position(angle);
            let is_selected = selected == Some(index);
            painter.circle_filled(position, if is_selected {4.5} else {1.7}, if is_selected {visuals.selection.bg_fill} else {visuals.text_color()});
        }
        let angle = dimension_angle_u128(base_dimension);
        let position = map.perm_position(angle);
        painter.circle_stroke(position, 7.0, egui::Stroke::new(1.0, visuals.text_color()));
    }

    pub fn draw_quantperm(painter:&egui::Painter, map:&SphereMap, heritage:&Heritage, visuals:&egui::Visuals){
        let network = heritage.transition.net_work;
        let activations = heritage.state.activations();
        let dimension = heritage.state.dimension();
        let angle = dimension_angle(dimension);
        let position = map.quantperm_position(angle);
        painter.circle_filled(position, 5.0, visuals.selection.bg_fill);
        painter.circle_stroke(position, 9.0, egui::Stroke::new(1.0, visuals.selection.stroke.color));
        let network_position = map.perm_position(angle);
        painter.line_segment([network_position, position], egui::Stroke::new(0.75, visuals.selection.stroke.color));
        painter.text(position + egui::vec2(10.0,10.0), egui::Align2::LEFT_TOP, "QuantPerm", egui::FontId::monospace(11.0), visuals.text_color());
        painter.text(map.center + egui::vec2(12.0,12.0), egui::Align2::LEFT_TOP, format!("network: {}", network), egui::FontId::monospace(10.0), visuals.weak_text_color());
        painter.text(map.center + egui::vec2(12.0,28.0), egui::Align2::LEFT_TOP, format!("activations: {}", activations), egui::FontId::monospace(10.0), visuals.weak_text_color());
    }

    pub fn handle_pointer(ui:&mut egui::Ui, map:&SphereMap, selected:&mut Option<usize>){
        let Some(pointer) = ui.input(|i| i.pointer.hover_pos()) else { return; };
        let dx = pointer.x - map.center.x; let dy = pointer.y - map.center.y; let distance = (dx*dx + dy*dy).sqrt();
        if distance > map.radius + 12.0 { return; }
        let mut angle = dy.atan2(dx); if angle < 0.0 { angle += TAU; }
        let index = ((angle / TAU) * DOMAIN_SIZE as f32).round() as usize % DOMAIN_SIZE;
        let position = map.perm_position(index as f32 * TAU / DOMAIN_SIZE as f32);
        let painter = ui.painter(); let visuals = ui.visuals();
        painter.circle_stroke(position, 7.0, egui::Stroke::new(1.0, visuals.selection.stroke.color));
        painter.text(position + egui::vec2(10.0,-10.0), egui::Align2::LEFT_TOP, format!("PERM {}", index), egui::FontId::monospace(11.0), visuals.text_color());
        if ui.input(|i| i.pointer.any_click()) { *selected = Some(index); }
    }

    pub fn draw_labels(painter:&egui::Painter, map:&SphereMap, heritage:&Heritage, visuals:&egui::Visuals){
        painter.text(map.center + egui::vec2(8.0, -map.radius - 20.0), egui::Align2::LEFT_CENTER, "PERM • vertical • authoritative • 360° • 2048", egui::FontId::monospace(11.0), visuals.text_color());
        painter.text(map.center + egui::vec2(8.0, map.radius * 0.30 + 14.0), egui::Align2::LEFT_CENTER, "QuantPerm • horizontal • activations", egui::FontId::monospace(11.0), visuals.selection.stroke.color);
        painter.text(map.center + egui::vec2(8.0, map.radius * 0.30 + 30.0), egui::Align2::LEFT_CENTER, format!("network={} • activations={}", heritage.transition.net_work, heritage.state.activations()), egui::FontId::monospace(10.0), visuals.weak_text_color());
    }

    pub fn draw_inspector(app:&PermDomainApp, ui:&mut egui::Ui){
        ui.heading("PERM / QuantPerm"); ui.separator();
        ui.label("Protocol"); ui.monospace(Perm::PROTOCOL);
        ui.label("Domain"); ui.monospace(format!("{} positions", Perm::DOMAIN_SIZE));
        ui.label("PERM"); ui.monospace("vertical / cryptographic / authoritative");
        ui.label("QuantPerm"); ui.monospace("horizontal / realized / Heritage-bound");
        ui.separator(); ui.label("Circumference"); ui.monospace("360°");
        ui.label("PERM units"); ui.monospace("2048");
        ui.label("Degrees / unit"); ui.monospace(format!("{:.9}°", 360.0 / DOMAIN_SIZE as f32));
        if let Some(index) = app.selected {
            let angle = index as f32 * TAU / DOMAIN_SIZE as f32;
            ui.separator(); ui.heading("Selected");
            ui.label("PERM index"); ui.monospace(index.to_string());
            ui.label("PERM angle"); ui.monospace(format!("{:.6}°", angle.to_degrees()));
            ui.label("PERM dimension"); ui.monospace(format!("{:032x}", app.perm.dimension()));
        }
        ui.separator(); ui.heading("QuantPerm Heritage");
        ui.label("Vertical / network"); ui.monospace(app.heritage.transition.net_work.to_string());
        ui.label("Horizontal / activations"); ui.monospace(app.heritage.state.activations().to_string());
        ui.label("Dimension"); ui.monospace(app.heritage.state.dimension().to_string());
        ui.label("Structural value"); ui.monospace(app.heritage.state.structural_value().to_string());
    }
}

// -----------------------------------------------------------------------------
// SPHERE MAP
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct SphereMap {
    pub center: egui::Pos2,
    pub radius: f32,
}

impl SphereMap {
    #[inline(always)]
    pub fn new(rect: egui::Rect) -> Self {
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.40;
        Self { center, radius }
    }

    #[inline(always)]
    pub fn center(&self) -> egui::Pos2 {
        self.center
    }

    #[inline(always)]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    #[inline(always)]
    pub fn perm_position(
        &self,
        angle: f32,
    ) -> egui::Pos2 {
        egui::pos2(
            self.center.x + angle.cos() * self.radius,
            self.center.y + angle.sin() * self.radius,
        )
    }

    #[inline(always)]
    pub fn quantperm_position(
        &self,
        angle: f32,
    ) -> egui::Pos2 {
        egui::pos2(
            self.center.x + angle.cos() * self.radius,
            self.center.y + angle.sin() * self.radius * 0.30,
        )
    }
}

#[inline(always)]
fn dimension_angle(
    dimension: u64,
) -> f32 {
    (
        dimension as f64
            / u64::MAX as f64
            * TAU as f64
    ) as f32
}

#[inline(always)]
fn dimension_angle_u128(
    dimension: u128,
) -> f32 {
    (
        dimension as f64
            / u128::MAX as f64
            * TAU as f64
    ) as f32
}