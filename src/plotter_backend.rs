use eframe::{egui::Ui, epaint::{Color32, Stroke, Pos2, PathShape, FontId}, emath::Align2};
use plotters_backend::{DrawingBackend, DrawingErrorKind, BackendColor, BackendStyle, BackendTextStyle, BackendCoord};
use plotters_backend::FontFamily as PlottersFontFamily;
use eframe::egui::FontFamily as EguiFontFamily;

pub struct EguiBackend<'a> {
    ui: &'a Ui,
}

impl<'a> EguiBackend<'a> {
    pub fn new(ui: &'a Ui) -> Self {
        Self {
            ui,
        }
    }
}

impl<'a> DrawingBackend for EguiBackend<'a> {
    type ErrorType = std::io::Error;

    fn get_size(&self) -> (u32, u32) {
        let bounds = self.ui.max_rect();
        (bounds.width() as u32, bounds.height() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
            &mut self,
            point: (i32, i32),
            color: BackendColor,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let (x0, y0) = point;

        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;

        x0 += bounds.min.x;
        y0 += bounds.min.y;

        let (x1, y1) = (x0 + 1.0, y0 + 1.0);

        let (r, g, b) = color.rgb;
        let a = (color.alpha * 255.0) as u8;
         
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let stroke = Stroke::new(1.0, color);

        painter.line_segment([Pos2{x: x0, y: y0}, Pos2{x: x1, y: y1}], stroke);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
            &mut self,
            from: (i32, i32),
            to: (i32, i32),
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let (x0, y0) = from;
        let (x1, y1) = to;

        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;
        let mut x1 = x1 as f32;
        let mut y1 = y1 as f32;

        x0 += bounds.min.x;
        y0 += bounds.min.y;
        x1 += bounds.min.x;
        y1 += bounds.min.y;

        let pos0 = Pos2 {x: x0, y: y0};
        let pos1 = Pos2 {x: x1, y: y1};

        let (r, g, b) = style.color().rgb;
        let a = (style.color().alpha * 255.0) as u8;
         
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        painter.line_segment([pos0, pos1], stroke);

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
            &mut self,
            text: &str,
            style: &TStyle,
            pos: (i32, i32),
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let (x, y) = pos;

        let mut x = x as f32;
        let mut y = y as f32;

        x += bounds.min.x;
        y += bounds.min.y;

        let pos = Pos2 {x, y};

        let font_size = style.size() as f32;
        let font_family = match style.family() {
            PlottersFontFamily::Serif | PlottersFontFamily::SansSerif => EguiFontFamily::Proportional,
            PlottersFontFamily::Monospace => EguiFontFamily::Monospace,
            PlottersFontFamily::Name(string) => EguiFontFamily::Name(string.into()),
        };

        let font = FontId {
            size: font_size,
            family: font_family,
        };

        let (r, g, b) = style.color().rgb;
        let a = (style.color().alpha * 255.0) as u8;
         
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        painter.text(pos, Align2::LEFT_TOP, text, font, color);

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
            &mut self,
            path: I,
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let points: Vec<Pos2> = path.into_iter().map(|point| {
            let (x, y) = point;

            let mut x = x as f32;
            let mut y = y as f32;

            x += bounds.min.x;
            y += bounds.min.y;

            Pos2 {x, y}
        }).collect();

        let (r, g, b) = style.color().rgb;
        
        let color = Color32::from_rgb(r, g, b);

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        let shape = PathShape::line(points, stroke);

        painter.add(shape);
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
            &mut self,
            vert: I,
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let points: Vec<Pos2> = vert.into_iter().map(|point| {
            let (x, y) = point;
            
            let mut x = x as f32;
            let mut y = y as f32;

            x += bounds.min.x;
            y += bounds.min.y;

            Pos2 {x, y}
        }).collect();

        let (r, g, b) = style.color().rgb;

        let a = (style.color().alpha * 255.0) as u8;
        
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let fill = color;

        let stroke = Stroke::NONE;   

        let shape = PathShape::convex_polygon(points, fill, stroke);

        painter.add(shape);

        Ok(())
    }
}
