use iced::{
    Background, Color, Element,
    widget::{Column, button, checkbox, column, container, row, text},
};

use crate::app::Message;

pub fn solution_ui(board: Option<[[usize; 9]; 9]>, size: f32) -> Element<'static, Message> {
    let mut ui = column![];
    for row in 0..9 {
        let mut row_ui = row![];
        for col in 0..9 {
            let mut dis = 0;
            if let Some(value) = board {
                dis = value[row][col];
            };
            row_ui = row_ui.push(
                button(text(format!("{}", dis)).size(size * 0.35).center())
                    .width(size)
                    .height(size),
            );
        }
        ui = ui.push(row_ui);
    }
    ui.into()
}

pub fn strategy_ui() -> Element<'static, Message> {
    row![checkbox("label", true),].into()
}
