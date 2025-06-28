use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(SudokuSolverApp::new(cc)))),
    );
}

#[derive(Default)]
struct SudokuSolverApp {
    cell_size: f32,
}

impl SudokuSolverApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { cell_size: 20.0 }
    }
}

impl eframe::App for SudokuSolverApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.add(egui::DragValue::new(&mut self.cell_size));
            for i in 1..10 {
                ui.horizontal(|ui| {
                    for j in 1..10 {
                        ui.add_sized(
                            egui::Vec2::splat(self.cell_size),
                            egui::Button::new(format!("{i},{j}")),
                        );
                    }
                });
            }
        });
    }
}
