#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::Arc, mem::size_of, time::{Instant, Duration}, cmp::Ordering, thread, f32::consts::PI};

use eframe::{egui::{self, TextureOptions, Painter, Rect}, epaint::{Shape, ColorImage, TextureHandle, Color32, Rounding, Pos2, Stroke, FontId, PathShape}, emath::Align2};
use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, PathElement, DrawingBackend, IntoLinspace, Rectangle}, style::{full_palette::{BLACK, RED, WHITE}, IntoFont, Color, BLUE}, series::{LineSeries, SurfaceSeries}};
use plotters_backend::*;
use plotters_backend::FontFamily as PlottersFontFamily;
use egui::FontFamily as EguiFontFamily;

static NAME: &str = "Ape-Aerotelem";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        NAME,
        options,
        Box::new(|_cc| Box::<Gui>::default()),
    )
}

struct Gui<> {
    //flight_plot_buf: Vec<u8>,
    ///// Width + Height
    //flight_plot_size: (u32, u32),
    //flight_plot_tex: Option<TextureHandle>,
}

impl<> Default for Gui<> {
    fn default() -> Self {
        //let flight_plot_buf = Vec::new();

        //Self {
        //    flight_plot_buf,
        //    flight_plot_size: (0, 0),
        //    flight_plot_tex: None,
        //}
        
        Self {  }
    }
}

impl<> eframe::App for Gui<> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let start_time = Instant::now();
        egui::TopBottomPanel::top("menubar").show(ctx, |ui|{
            egui::menu::bar(ui, |bar| {
                bar.menu_button("File", |mb| {
                    if mb.button("Open").clicked() {
                        todo!()
                    }
                    
                    if mb.button("Export").clicked() {
                        todo!()
                    }

                    if mb.button("Statistics").clicked() {
                        todo!()
                    }

                    if mb.button("Preferences").clicked() {
                        todo!()
                    }

                    if mb.button("Close").clicked() {
                        todo!()
                    }
                });

                bar.menu_button("View", |mb| {
                    if mb.button("Zoom In").clicked() {
                        todo!()
                    }
                    
                    if mb.button("Zoom Out").clicked() {
                        todo!()
                    }

                    if mb.button("Re-Center").clicked() {
                        todo!()
                    }
                });
                
                bar.menu_button("Help", |mb| {
                    if mb.button("Quick Guide").clicked() {
                        todo!()
                    }
                    
                    if mb.button("Manual").clicked() {
                        todo!()
                    }

                    if mb.button("About").clicked() {
                        todo!()
                    }
                });
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();

            let width = rect.width() as u32;
            let height = rect.height() as u32;

            //if (width, height) != self.flight_plot_size {
            //    self.flight_plot_size = (width, height);

            //    // Multiply by 3 since each pixel is 3 bytes
            //    let size = (width * height * 3) as usize;
            //    self.flight_plot_buf = vec![0; size];
            //}

            //let backend = BitMapBackend::with_buffer(&mut self.flight_plot_buf, self.flight_plot_size).into_drawing_area();
            
            let backend = PainterBackend::new(ui.painter(), rect).into_drawing_area();

            backend.fill(&WHITE).unwrap();
            let x_axis = (-3.0..3.0).step(0.1);
            let z_axis = (-3.0..3.0).step(0.1);

            let mut chart = ChartBuilder::on(&backend)
                .caption(format!("3D Plot Test"), ("sans-serif", 20).into_font())
                .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone()).unwrap();

            chart.with_projection(|mut pb| {
                pb.yaw = 0.5;
                pb.scale = 0.9;
                pb.into_matrix()
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw().unwrap();

            chart
                .draw_series(
                    SurfaceSeries::xoz(
                        (-30..30).map(|f| f as f64 / 10.0),
                        (-30..30).map(|f| f as f64 / 10.0),
                        |x, z| (x * x + z * z).cos(),
                    )
                    .style(BLUE.mix(0.2).filled()),
                ).unwrap()
                .label("Surface")
                .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

            chart
                .draw_series(LineSeries::new(
                    (-100..100)
                        .map(|y| y as f64 / 40.0)
                        .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                    &BLACK,
                )).unwrap()
                .label("Line")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

            chart
                .configure_series_labels()
                .border_style(&BLACK)
                .draw().unwrap();

            backend.present().unwrap();

            // Drop the chart and backend so we can borrow the buffer
            // drop(chart);
            // drop(backend);

            // let render = ColorImage::from_rgb([width as usize, height as usize], &self.flight_plot_buf);

            // let texture = self.flight_plot_tex.get_or_insert_with(|| {
            //     ui.ctx().load_texture("flight-plot-tex", render.clone(), Default::default())
            // });

            // let options = TextureOptions::default();

            // texture.set(render, options);

            // ui.image(texture, [width as f32, height as f32]);
        });

        egui::TopBottomPanel::bottom("render_time").show(ctx, |ui| {
            ui.label(format!("Render Time: {}", start_time.elapsed().as_micros()));
        });
    }
}

struct PainterBackend<'a> {
    painter: &'a Painter,
    bounds: Rect,
}

impl<'a> PainterBackend<'a> {
    pub fn new(painter: &'a Painter, bounds: Rect) -> Self {
        Self {
            painter,
            bounds
        }
    }
}

impl<'a> DrawingBackend for PainterBackend<'a> {
    type ErrorType = std::io::Error;

    fn get_size(&self) -> (u32, u32) {
        (self.bounds.width() as u32, self.bounds.height() as u32)
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

        let (x0, y0) = point;

        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;

        x0 += self.bounds.min.x;
        y0 += self.bounds.min.y;

        let (x1, y1) = (x0 + 1.0, y0 + 1.0);

        let (r, g, b) = color.rgb;
        let a = (color.alpha * 255.0) as u8;
         
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let stroke = Stroke::new(1.0, color);

        self.painter.line_segment([Pos2{x: x0, y: y0}, Pos2{x: x1, y: y1}], stroke);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
            &mut self,
            from: (i32, i32),
            to: (i32, i32),
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {

        let (x0, y0) = from;
        let (x1, y1) = to;

        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;
        let mut x1 = x1 as f32;
        let mut y1 = y1 as f32;

        x0 += self.bounds.min.x;
        y0 += self.bounds.min.y;
        x1 += self.bounds.min.x;
        y1 += self.bounds.min.y;

        let pos0 = Pos2 {x: x0, y: y0};
        let pos1 = Pos2 {x: x1, y: y1};

        let (r, g, b) = style.color().rgb;
        let a = (style.color().alpha * 255.0) as u8;
         
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        self.painter.line_segment([pos0, pos1], stroke);

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
            &mut self,
            text: &str,
            style: &TStyle,
            pos: (i32, i32),
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {

        let (x, y) = pos;

        let mut x = x as f32;
        let mut y = y as f32;

        x += self.bounds.min.x;
        y += self.bounds.min.y;

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

        self.painter.text(pos, Align2::LEFT_TOP, text, font, color);

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
            &mut self,
            path: I,
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {

        let points: Vec<Pos2> = path.into_iter().map(|point| {
            let (x, y) = point;

            let mut x = x as f32;
            let mut y = y as f32;

            x += self.bounds.min.x;
            y += self.bounds.min.y;

            Pos2 {x, y}
        }).collect();

        let (r, g, b) = style.color().rgb;
        
        let color = Color32::from_rgb(r, g, b);

        let stroke = Stroke::new(style.stroke_width() as f32, color.clone());

        let shape = PathShape::line(points, stroke);

        self.painter.add(shape);
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
            &mut self,
            vert: I,
            style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {

        let mut points: Vec<Pos2> = vert.into_iter().map(|point| {
            let (x, y) = point;
            
            let mut x = x as f32;
            let mut y = y as f32;

            x += self.bounds.min.x;
            y += self.bounds.min.y;

            Pos2 {x, y}
        }).collect();

        // Make them clockwise. Plotter has a habit of making them not clockwise,
        // or in order for that matter. Very unusual.
        let mut center_x = 0.0;
        let mut center_y = 0.0;

        for point in &points {
            center_x += point.x * (1.0/(points.len() as f32));
            center_y += point.y * (1.0/(points.len() as f32));
        }

        points.sort_by(|a, b| {
            let angle1 = (a.x - center_x).atan2(a.y - center_y);
            let angle2 = (b.x - center_x).atan2(b.y - center_y);
            
            angle2.partial_cmp(&angle1).unwrap()
        });

        // Do not print a polygon if there is an invalid internal angle
        for (i, point) in points[..points.len()-1].iter().enumerate() {
            let angle1 = (point.x - center_x).atan2(point.y - center_y);

            let point = points[i + 1];

            let angle2 = (point.x - center_x).atan2(point.y - center_y);

            if (angle1 - angle2).abs() < 0.01 * PI{
                return Ok(())
            }
        }

        let (r, g, b) = style.color().rgb;

        let a = (style.color().alpha * 255.0) as u8;
        
        let color = Color32::from_rgba_unmultiplied(r, g, b, a);

        let fill = color.clone();

        let stroke = Stroke::NONE;   

        let shape = PathShape::convex_polygon(points, fill, stroke);

        self.painter.add(shape);

        Ok(())
    }
}
