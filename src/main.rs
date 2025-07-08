use eframe::egui;
use reqwest::blocking::{Client, get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;

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
    board: Arc<Mutex<[[usize; 9]; 9]>>,
    puzzle: [[usize; 9]; 9],
    solution: [[usize; 9]; 9],
    difficulty: String,
    show_potentials: bool,
    potentials: [[u16; 9]; 9],
}

impl SudokuSolverApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: Arc::new(Mutex::new([[0; 9]; 9])),
            ..Default::default()
        }
    }
    fn get_board_yds(&mut self, difficulty: String) {
        let client = Client::new();
        let request_body = HttpYouDoSudokuRequest {
            difficulty,
            solution: true,
            array: false,
        };
        let response: HttpYouDoSudokuResponse = client
            .post("https://youdosudoku.com/api")
            .json(&request_body)
            .send()
            .unwrap()
            .json()
            .unwrap();

        //println!("{:#?}", response);

        // Since YouDoSudoku encodes their puzzles as a single String with 81 characters,
        // we will have to do some manual parsing which we can take the chars ascii equivalent since its just numbers.
        // They do have support for arrays but i'll
        // explore that in the future.
        for (i, c) in response.puzzle.chars().enumerate() {
            let row = i / 9 as usize;
            let col = i % 9 as usize;
            let digit = c as usize - '0' as usize;
            self.puzzle[row][col] = digit;
            self.potentials[row][col] = if digit>0 { 1 << (digit - 1) } else {0b111111111};
        }
        for (i, c) in response.solution.chars().enumerate() {
            let row = i / 9 as usize;
            let col = i % 9 as usize;
            self.solution[row][col] = c as usize - '0' as usize;
        }
        self.difficulty = response.difficulty;
        *self.board.lock().unwrap() = self.puzzle;
    }
    fn brute_force(&mut self, ctx: &egui::Context) {
        let board = self.board.clone();
        let puzzle = self.puzzle.clone();
        thread::spawn(move || {
            let mut i = 0;
            let mut dir = 1;
            while i < 81 {
                let row = i / 9 as usize;
                let col = i % 9 as usize;
                let cell_row = row / 3 as usize;
                let cell_col = col / 3 as usize;
                //println!("element {i} cell row {cell_row} cell col {cell_col}");

                if puzzle[row][col] == 0 {
                    loop {
                        thread::sleep(std::time::Duration::from_millis(200));
                        let mut b = { board.lock().unwrap() };
                        b[row][col] += 1;
                        let digit = b[row][col];
                        //println!("{digit}");
                        if digit == 10 {
                            b[row][col] = 0;
                            i -= 1;
                            dir = -1;
                            break;
                        }
                        let col_check = b
                            .iter()
                            .map(|r| r[col])
                            .filter(|&e| e == b[row][col])
                            .collect::<Vec<_>>()
                            .len();
                        let row_check = b[row]
                            .iter()
                            .filter(|&e| e == &b[row][col])
                            .collect::<Vec<_>>()
                            .len();
                        let cr = cell_row * 3;
                        let cc = cell_col * 3;
                        let cell_check = b[cr..cr + 3]
                            .iter()
                            .flat_map(|r| r[cc..cc + 3].iter().collect::<Vec<_>>())
                            .filter(|&e| e == &b[row][col])
                            .collect::<Vec<_>>()
                            .len();
                        //println!("COL {:?} {}", b.iter().map(|r| r[col]).collect::<Vec<_>>(), col_check);
                        //println!("ROW {:?} {}", b[row], row_check);
                        //println!("CELL {:?} {}", b[cr..cr+3].iter().flat_map(|r| r[cc..cc+3].iter().collect::<Vec<_>>()).collect::<Vec<_>>(), cell_check);

                        if row_check == 1 && col_check == 1 && cell_check == 1 {
                            i += 1;
                            dir = 1;
                            break;
                        }
                    }
                } else {
                    if i == 0 {
                        dir = 1;
                    }
                    if dir == 1 {
                        i += 1;
                    } else if dir == -1 {
                        i -= 1;
                    }
                }
            }
            println!("Solved?");
        });
    }
    fn update_potentials(&mut self) {
        let mut i = 0;
        while i < 81 {
            let row = i / 9 as usize;
            let col = i % 9 as usize;
            let cell_row = row / 3 as usize;
            let cell_col = col / 3 as usize;
            //println!("element {i} cell row {cell_row} cell col {cell_col}");
            let board = { self.board.lock().unwrap() };

            if board[row][col] == 0 {
                let col_check = board
                    .iter()
                    .map(|r| r[col])
                    .filter(|&e| e != 0)
                    .collect::<Vec<_>>();
                    
                //println!("{col_check:?}");
                for num in col_check {
                    self.potentials[row][col] &= !( 1 << (num-1));
                }
                let row_check = board[row]
                    .iter()
                    .filter(|&e| *e != 0)
                    .collect::<Vec<_>>();
                //println!("{row_check:?}");
                for num in row_check {
                    self.potentials[row][col] &= !( 1 << (num-1));
                }
                let cr = cell_row * 3;
                let cc = cell_col * 3;
                let cell_check = board[cr..cr + 3]
                    .iter()
                    .flat_map(|r| r[cc..cc + 3].iter().collect::<Vec<_>>())
                    .filter(|&e| *e != 0)
                    .collect::<Vec<_>>();
                //println!("{cell_check:?}");
                for num in cell_check {
                    self.potentials[row][col] &= !( 1 << (num-1));
                }
            }
            i+=1;
        }
        println!("Updated Potentials");
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

#[derive(Debug, Serialize)]
struct HttpYouDoSudokuRequest {
    difficulty: String,
    solution: bool,
    array: bool,
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
                self.puzzle = response.newboard.grids[0].value;
                *self.board.lock().unwrap() = self.puzzle;
                self.difficulty = response.newboard.grids[0].difficulty.clone();
            }
            if ui.button("Get Random Board\n(YouDoSudoku API)").clicked() {
                let response: HttpYouDoSudokuResponse =
                    get("https://you-do-sudoku-api.vercel.app/api")
                        .unwrap()
                        .json()
                        .unwrap();
                println!("{:#?}", response);

                // Since YouDoSudoku encodes their puzzles as a single String with 81 characters,
                // we will have to do some manual parsing which we can take the chars ascii equivalent since its just numbers.
                // They do have support for arrays but i'll
                // explore that in the future.
                for (i, c) in response.puzzle.chars().enumerate() {
                    let row = i / 9 as usize;
                    let col = i % 9 as usize;
                    self.puzzle[row][col] = c as usize - '0' as usize;
                }
                for (i, c) in response.solution.chars().enumerate() {
                    let row = i / 9 as usize;
                    let col = i % 9 as usize;
                    self.solution[row][col] = c as usize - '0' as usize;
                }
                self.difficulty = response.difficulty;
                *self.board.lock().unwrap() = self.puzzle;
            }
            if ui.button("Easy Board\n(YouDoSudoku API)").clicked() {
                self.get_board_yds("easy".into());
            }
            if ui.button("Medium Board\n(YouDoSudoku API)").clicked() {
                self.get_board_yds("medium".into());
            }
            if ui.button("Hard Board\n(YouDoSudoku API)").clicked() {
                self.get_board_yds("hard".into());
            }
            ui.separator();
            if ui.button("Brute Force").clicked() {
                self.brute_force(&ctx);
            }
            ui.separator();
            ui.checkbox(&mut self.show_potentials, "Show Potentials");
            if ui.button("Update Potentials").clicked() {
                self.update_potentials();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    *self.board.lock().unwrap() = self.puzzle;
                }
                ui.label(format!("Difficulty: {}", self.difficulty));
                ui.add_space(40.0);
                if *self.board.lock().unwrap() == self.solution && self.solution[0][0] != 0 {
                    ui.label("Won");
                }
            });

            ui.group(|ui| {
                let cell_size = ui.available_width().min(ui.available_height()) / 11.0;
                for i in 0..9 {
                    ui.horizontal(|ui| {
                        for j in 0..9 {
                            let digit = self.board.lock().unwrap()[i][j];
                            let puzzle_digit = self.puzzle[i][j];
                            let solution_digit = self.solution[i][j];
                            let potential = self.potentials[i][j];
                            let cell = egui::Button::new(if digit != 0 {
                                digit.to_string()
                            } else {
                                // TODO: Is there a better way to do this?
                                if self.show_potentials {
                                    let fmt = format!(
                                        "{} {} {}\n{} {} {}\n{} {} {}",
                                        (potential & (1 << 0) == (1 << 0)) as usize *1,
                                        (potential & (1 << 1) == (1 << 1)) as usize *2,
                                        (potential & (1 << 2) == (1 << 2)) as usize *3,
                                        (potential & (1 << 3) == (1 << 3)) as usize *4,
                                        (potential & (1 << 4) == (1 << 4)) as usize *5,
                                        (potential & (1 << 5) == (1 << 5)) as usize *6,
                                        (potential & (1 << 6) == (1 << 6)) as usize *7,
                                        (potential & (1 << 7) == (1 << 7)) as usize *8,
                                        (potential & (1 << 8) == (1 << 8)) as usize *9,
                                    );
                                    fmt
                                } else {
                                    "".to_string()
                                }
                            });
                            // Disable the cells that belong in the puzzle
                            ui.add_enabled_ui(puzzle_digit == 0, |ui| {
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
                                            for n in 0..10 {
                                                if ui.button(format!("\t{n}\t")).clicked() {
                                                    self.board.lock().unwrap()[i][j] = n;
                                                }
                                                if n == 0 { ui.end_row(); continue;}
                                                if n % 3 == 0 {
                                                    ui.end_row();
                                                }
                                            }
                                        });
                                    },
                                );
                            });
                            if (j + 1) % 3 == 0 && j != 8 {
                                ui.separator();
                            }
                        }
                    });
                    // The length of this separator is extending longer than the cells.
                    // Temporarily set a size for the separator and guess the length ratio
                    // it should be
                    if (i + 1) % 3 == 0 && i != 8 {
                        ui.add_sized([cell_size * 10.85, 0.5], egui::Separator::default());
                    }
                }
            });
        });
        ctx.request_repaint();
    }
}
