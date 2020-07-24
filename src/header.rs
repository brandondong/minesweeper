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
    div![
        button![
            C!["Header_refreshButton"],
            svg![
                C!["Header_buttonSvg"],
                attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
                path![attrs! {At::Stroke => "none", At::D => "M0 0h24v24H0z"}],
                path![attrs! {At::D => "M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5"}],
                path![attrs! {At::D => "M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5"}]
            ]
        ],
        button![svg![
            C!["Header_buttonSvg"],
            attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
            path![attrs! {At::Stroke => "none", At::D => "M0 0h24v24H0z"}],
            path![attrs! {At::D => "M3 12h1M12 3v1M20 12h1M5.6 5.6l.7 .7M18.4 5.6l-.7 .7"}],
            path![
                attrs! {At::D => "M9 16a5 5 0 1 1 6 0a3.5 3.5 0 0 0 -1 3a2 2 0 0 1 -4 0a3.5 3.5 0 0 0 -1 -3"}
            ],
            line_![attrs! {At::X1 => "9.7", At::Y1 => "17", At::X2 => "14.3", At::Y2 => "17"}]
        ]]
    ]
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
