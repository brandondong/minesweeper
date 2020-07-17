use crate::{Board, HiddenState, Model, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_header(model: &Model) -> Node<Msg> {
    div![
        C!["Header_container"],
        div![C!["Header_section"], view_difficulty_select(model)],
        div![
            C!["Header_section", "Header_centerAligned"],
            model.board.flags_left()
        ],
        div![
            C!["Header_section", "Header_rightAligned"],
            view_action_buttons(model)
        ],
    ]
}

fn view_difficulty_select(model: &Model) -> Node<Msg> {
    let easy_selected = matches!(model.board, Board::Easy(_));
    let medium_selected = matches!(model.board, Board::Medium(_));
    let hard_selected = matches!(model.board, Board::Hard(_));
    select![
        el_ref(&model.select_difficulty_element),
        ev(Ev::Change, |_| {
            Msg::DifficultyChange(header_select_mapping)
        }),
        option![attrs! {At::Selected => easy_selected.as_at_value()}, "Easy"],
        option![
            attrs! {At::Selected => medium_selected.as_at_value()},
            "Medium"
        ],
        option![attrs! {At::Selected => hard_selected.as_at_value()}, "Hard"]
    ]
}

fn view_action_buttons(_model: &Model) -> Node<Msg> {
    div![button!["Refresh"], button!["Hint"]]
}

fn header_select_mapping(index: i32) -> Board {
    match index {
        0 => Board::Easy(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 10 * 8],
        )),
        1 => Board::Medium(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 18 * 14],
        )),
        _ => Board::Hard(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 24 * 20],
        )),
    }
}
