use std::thread;
use std::time::Duration;

use crate::api;
use crate::solver;
use crate::sudoku::{Difficulty, Sudoku};
use crate::ui;

use iced::futures::SinkExt;
use iced::futures::Stream;
use iced::stream::try_channel;
use iced::widget::progress_bar;
use iced::widget::toggler;
use iced::widget::{Column, Space, Tooltip, button, column, container, row, text, tooltip};
use iced::{Subscription, Task, window};

pub struct App {
    sudoku: Sudoku,
    selected_cell: Option<(usize, usize)>,
    // Internals
    window_size: (f32, f32),
    view_solution: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Board Utility
    SelectedCell(usize, usize),
    SelectedNumber(usize),
    SelectedDifficulty(Difficulty),
    SetBoard([[usize; 9]; 9]),
    ClearBoard,
    // Solvers
    BruteForce,
    // Internals
    WindowResize(f32, f32),
    ViewSolution(bool),
    Error,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            sudoku: Sudoku::new(),
            selected_cell: None,
            // Internals
            window_size: (600.0, 600.0),
            view_solution: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectedCell(row, col) => {
                self.selected_cell = Some((row, col));
                Task::none()
            }
            Message::SelectedNumber(num) => {
                if let Some((row, col)) = self.selected_cell {
                    self.sudoku.update_cell(row, col, num);
                    self.selected_cell = None;
                }
                Task::none()
            }
            Message::SelectedDifficulty(diff) => {
                let api_diff = match diff {
                    Difficulty::Easy => "easy",
                    Difficulty::Medium => "medium",
                    Difficulty::Hard => "hard",
                };
                let (puzzle, solution) = api::get_board(api_diff.to_string());
                //println!("got {:?}", puzzle);
                self.sudoku.set_board(puzzle);
                self.sudoku.set_puzzle(puzzle);
                self.sudoku.set_solution(solution);
                self.sudoku.set_difficulty(Some(diff));
                Task::none()
            }
            Message::SetBoard(board) => {
                self.sudoku.set_board(board);
                Task::none()
            }
            Message::ClearBoard => {
                self.sudoku.clear_board();
                Task::none()
            }
            // Solvers
            Message::BruteForce => {
                let puzzle = match self.sudoku.get_puzzle() {
                    Some(puzzle) => puzzle,
                    _ => [[0; 9]; 9],
                };
                Task::run(
                    solver::brute_force(self.sudoku.get_board(), puzzle),
                    |r: Result<[[usize; 9]; 9], ()>| match r {
                        Ok(board) => Message::SetBoard(board),
                        Err(_) => Message::Error, // we could handle errors here if we want
                    },
                )
            }
            // Internals
            Message::WindowResize(width, height) => {
                self.window_size = (width, height);
                Task::none()
            }
            Message::ViewSolution(value) => {
                self.view_solution = value;
                Task::none()
            }
            Message::Error => {
                println!("Error");
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Column<'_, Message> {
        let (width, height) = self.window_size;

        let mut difficulty_row = row![];
        for (diff, caller) in [
            ("Easy", Difficulty::Easy),
            ("Medium", Difficulty::Medium),
            ("Hard", Difficulty::Hard),
        ] {
            let btn = button(diff).on_press(Message::SelectedDifficulty(caller));
            difficulty_row = difficulty_row.push(btn);
        }

        let solver_bar = column![
            "Solvers",
            button("Brute Force").on_press(Message::BruteForce),
            "Strategies",
            ui::strategy_ui(),
            ui::strategy_ui(),
            ui::strategy_ui(),
            ui::strategy_ui(),
        ]
        .spacing(10)
        .padding(5)
        .width(width * 0.2)
        .align_x(iced::Center);

        let sidebar = column![
            "Select Level",
            difficulty_row.spacing(5).wrap(),
            button("Clear").on_press(Message::ClearBoard),
            toggler(self.view_solution)
                .label("View Solution")
                .on_toggle(Message::ViewSolution),
            ui::solution_ui(self.sudoku.get_solution(), width * 0.2 / 9.0)
        ]
        .spacing(10)
        .padding(5)
        .width(width * 0.2)
        .align_x(iced::Center);

        let lbldiff = match self.sudoku.get_difficulty() {
            Some(diff) if diff == Difficulty::Easy => "Easy",
            Some(diff) if diff == Difficulty::Medium => "Medium",
            Some(diff) if diff == Difficulty::Hard => "Hard",
            _ => "Select a difficulty to play",
        };

        let board_headers = column![row!["Difficulty:", Space::with_width(20), lbldiff]];

        let board = self.sudoku.get_board();
        let puzzle = self.sudoku.get_puzzle();
        let cell_width = width * 0.6 / 9.0;
        let cell_height = height / 9.0;
        let cell_size = cell_height.min(cell_width);
        //println!("{} ; {}", width, height);

        let mut ui_col = column![];
        for row in 0..9 {
            let mut ui_row = row![];
            for col in 0..9 {
                let content = if self.selected_cell == Some((row, col)) {
                    let mut ui_num_col = column![];
                    let num_cell_size = cell_size / 3.0;
                    for i_row in 0..3 {
                        let mut ui_num_row = row![];
                        for i_col in 0..3 {
                            let num = i_row * 3 + i_col + 1;
                            let mut num_btn = button(text(num).size(num_cell_size * 0.5).center())
                                .on_press(Message::SelectedNumber(num))
                                .height(num_cell_size)
                                .width(num_cell_size);

                            ui_num_row = ui_num_row.push(num_btn);
                        }
                        ui_num_col = ui_num_col.push(ui_num_row);
                    }
                    ui_num_col.into()
                } else {
                    let num = board[row][col];
                    let mut btn = button(
                        text(if num == 0 {
                            "".to_string()
                        } else {
                            num.to_string()
                        })
                        .size(cell_size * 0.5)
                        .center(),
                    )
                    .height(cell_size)
                    .width(cell_size);
                    if let Some(puz) = puzzle {
                        if puz[row][col] == 0 {
                            btn = btn.on_press(Message::SelectedCell(row, col))
                        }
                    } else {
                        btn = btn.on_press(Message::SelectedCell(row, col))
                    }
                    column![btn]
                };
                if col > 0 && (col) % 3 == 0 {
                    ui_row = ui_row.push(Space::new(1, cell_size));
                }
                ui_row = ui_row.push(content);
            }
            if row > 0 && (row) % 3 == 0 {
                ui_col = ui_col.push(Space::new(cell_size, 1));
            }
            ui_col = ui_col.push(ui_row);
        }
        column![row![sidebar, column![board_headers, ui_col], solver_bar]]
    }
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, id| match event {
            iced::Event::Window(window::Event::Resized(size)) => {
                Some(Message::WindowResize(size.width, size.height))
            }
            _ => None,
        })
    }
}
