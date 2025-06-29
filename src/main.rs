use eframe::egui;
use reqwest::blocking::get;
use serde::Deserialize;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sudoku Solver",
        native_options,
        Box::new(|cc| Ok(Box::new(SudokuSolverApp::new(cc)))),
    );
}

#[derive(Default)]
struct SudokuSolverApp {
    board: [[usize; 9]; 9],
    solution: [[usize; 9]; 9],
    difficulty: String,
}

impl SudokuSolverApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { board: [[0; 9]; 9], 
        ..Default::default()}
    }
}

#[derive(Debug, Deserialize)]
struct HttpDosukuResponse {
    newboard: NewBoardResponse,
}
#[derive(Debug, Deserialize)]
struct NewBoardResponse {
    grids: Vec<BoardResponse>,
    results: usize,
    message: String,
}
#[derive(Debug, Deserialize)]
struct BoardResponse {
    value: [[usize; 9]; 9],
    solution: [[usize; 9]; 9],
    difficulty: String,
}

#[derive(Debug, Deserialize)]
struct HttpYouDoSudokuResponse {
    difficulty: String, 
    puzzle: String,
    solution: String,
}


impl eframe::App for SudokuSolverApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            if ui.button("Get Random Board\n(Dosuku API)").clicked() {
                // Dosuku only has the ability to query the number of boards to request.
                // If a specific difficulty is requested then more time would be spent cycling 
                // requests until desired difficulty is requested.
                // The Official API for this has an open issue on this but does not look like it 
                // will be resolved any time soon.
                //
                // I'll leave this here for random board generation with a solution.
                let response: HttpDosukuResponse = get("https://sudoku-api.vercel.app/api/dosuku")
                    .unwrap()
                    .json()
                    .unwrap();
                //println!("{:#?}", response.newboard.grids[0].value);
                self.board = response.newboard.grids[0].value;
                self.difficulty = response.newboard.grids[0].difficulty.clone();
            }
            if ui.button("Get Random Board\n(YouDoSudoku API)").clicked() {
                let response: HttpYouDoSudokuResponse = get("https://you-do-sudoku-api.vercel.app/api")
                    .unwrap()
                    .json()
                    .unwrap();
                println!("{:#?}", response);

                // Since YouDoSudoku encodes their puzzles as a single String with 81 characters,
                // we will have to do some manual parsing which we can take the chars ascii equivalent since its just numbers. 
                // They do have support for arrays but i'll 
                // explore that in the future.
                for (i,c) in response.puzzle.chars().enumerate() {
                    let row = i / 9 as usize;
                    let col = i % 9 as usize;
                    self.board[row][col] = c as usize - '0' as usize;
                }
                for (i,c) in response.solution.chars().enumerate() {
                    let row = i / 9 as usize;
                    let col = i % 9 as usize;
                    self.solution[row][col] = c as usize - '0' as usize;
                }
                self.difficulty = response.difficulty;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Difficulty: {}", self.difficulty));
                if self.board == self.solution && self.solution[0][0] != 0 { ui.label("Won"); }
            });

            ui.group(|ui| {
                let cell_size = ui.available_width().min(ui.available_height()) / 10.0;
                for i in 0..9 {
                    ui.horizontal(|ui| {
                        for j in 0..9 {
                            let digit = self.board[i][j];
                            let cell = egui::Button::new(if digit != 0 {
                                digit.to_string()
                            } else {
                                "".to_string()
                            });
                            let popup_id = ui.make_persistent_id(format!("{i}{j}"));
                            let response = ui.add_sized(egui::Vec2::splat(cell_size), cell);
                            if response.clicked() {
                                ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                            }
                            let below = egui::AboveOrBelow::Below;
                            let close_on_click_outside =
                                egui::popup::PopupCloseBehavior::CloseOnClick;
                            egui::popup::popup_above_or_below_widget(
                                ui,
                                popup_id,
                                &response,
                                below,
                                close_on_click_outside,
                                |ui| {
                                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                                        for n in 1..10 {
                                            if ui.button(format!("\t{n}\t")).clicked() {
                                                self.board[i][j] = n;
                                            }
                                            if n % 3 == 0 {
                                                ui.end_row();
                                            }
                                        }
                                    });
                                },
                            );
                        }
                    });
                }
            });
        });
    }
}
