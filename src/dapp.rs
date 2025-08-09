use crate::api;
use crate::sudoku::Sudoku;
use dioxus::prelude::*;
static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone)]
struct AppSettings {
    sidebar_open: bool,
}
impl AppSettings {
    fn new() -> Self {
        AppSettings {
            sidebar_open: false,
        }
    }
}

#[component]
pub fn App() -> Element {
    let settings = use_context_provider(|| Signal::new(AppSettings::new()));
    let sudoku = use_context_provider(|| Signal::new(Sudoku::new()));
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        MenuBar {}
        // div {
        //     class: "",
        //     if settings().sidebar_open { SideBarLeft {} }
        Board{}
        // }
        // SideBarRight {}
        // BottomBar {}
    }
}

#[component]
fn MenuBar() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut sudoku = use_context::<Signal<Sudoku>>();
    let sidebar_toggle = move |_| {
        if settings().sidebar_open {
            settings.write().sidebar_open = false;
        } else {
            settings.write().sidebar_open = true;
        }
    };
    let mut difficulty = use_signal(|| "");

    let api_board = move |_| {
        let difficulty = difficulty.read().to_string();
        spawn(async move {
            let result =
                tokio::task::spawn_blocking(move || api::get_board(difficulty.to_lowercase()))
                    .await
                    .unwrap();
            sudoku.write().set_board(result.0);
            sudoku.write().set_solution(result.1);
        });
    };

    rsx! {
        div {
            class:"navbar bg-base-300 shadow-sm",
          a { class:"btn btn-ghost text-xl",
              onclick: sidebar_toggle,
              "{settings().sidebar_open}"
          }
          for diff in ["Easy", "Medium", "Hard"] {
            button { class:"btn btn-ghost text-md",
                onclick: {
                    difficulty.set(diff);
                    api_board
                },
                "{diff}"
            }
          }
        }
    }
}

#[component]
fn SideBarLeft() -> Element {
    let settings = use_context::<Signal<AppSettings>>();
    rsx! {
        div { class: "inline",
            ul { class: "menu bg-base-200  w-40 p-4",
                li { class: "menu-title", "Choose Level" }
                li {
                    a { "Easy" }
                }
                li {
                    a { "Medium" }
                }
                li {
                    a { "Hard" }
                }
                li { class: "menu-title", "Future Items" }
                li {
                    a { "Clear" }
                }
                li {
                    a { "Load Puzzle From Input/File" }
                }
                li {
                    a { "Load Previous Puzzle from DB" }
                }
                li {
                    a { "View Solution" }
                }
            }
        }
    }
}

#[component]
fn Board() -> Element {
    let sudoku = use_context::<Signal<Sudoku>>();
    let board = sudoku.read().get_board();
    rsx! {
        div {
            class: "size-fit m-auto grid grid-cols-3 grid-rows-3 gap-1 rounded-xl p-2 bg-base-100",
            for block in 0..9 {
                div { class: "size-40 grid grid-cols-3 grid-rows-3",
                    for cell in 0..9 {
                        button {
                            class:"bg-indigo-600 rounded-sm",
                            "{board[block / 3 as usize * 3 + cell / 3 as usize][block % 3 as usize * 3 + cell % 3 as usize]}" }
                    }
                }
            }
        }
    }
}

#[component]
fn CellButton(row: usize, col: usize) -> Element {
    rsx! {}
}
