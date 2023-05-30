#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::Arc, mem::size_of, time::{Instant, Duration}, cmp::Ordering, thread, f32::consts::PI};

use eframe::{egui::{self, TextureOptions, Painter, Rect}, epaint::{Shape, ColorImage, TextureHandle, Color32, Rounding, Pos2, Stroke, FontId, PathShape, Vec2}, emath::Align2, CreationContext};
use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, PathElement, DrawingBackend, IntoLinspace, Rectangle}, style::{full_palette::{BLACK, RED, WHITE}, IntoFont, Color, BLUE}, series::{LineSeries, SurfaceSeries}};
use plotter_backend::EguiBackend;

mod plotter_backend;

static NAME: &str = "Ape-Aerotelem";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        min_window_size: Some(Vec2::new(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        NAME,
        options,
        Box::new(|cc| Box::new(Gui::new(cc))),
    )
}

struct Gui {
    tabstate: usize,
}

impl Gui {
    fn new(cc: &CreationContext) -> Self {
        // Disable feathering to combat artifacts caused by 3d plots
        cc.egui_ctx.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });
        Self {tabstate: 0}
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
            
            let backend = EguiBackend::new(ui).into_drawing_area();

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
        });

        egui::TopBottomPanel::bottom("render_time").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tabstate, 0, "X");
                ui.selectable_value(&mut self.tabstate, 1, "Y");
                ui.selectable_value(&mut self.tabstate, 2, "Z");
            });
            ui.label(format!("Render Time: {}", start_time.elapsed().as_micros()));
        });
    }
}

