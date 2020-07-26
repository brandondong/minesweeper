use crate::icons::{view_flag_icon, view_hint_icon, view_reset_icon, view_timer_icon};
use crate::{Board, HiddenState, Model, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_header(model: &Model) -> Node<Msg> {
    div![
        C!["Header_container"],
        div![C!["Header_edgeSection"], view_difficulty_select(model)],
        div![
            C!["Header_middleSection", "Header_centerAligned"],
            view_game_info(model)
        ],
        div![
            C!["Header_edgeSection", "Header_rightAligned"],
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

fn view_game_info(model: &Model) -> Node<Msg> {
    div![
        C!["Header_gameInfoContainer"],
        div![
            C!["Header_flagSectionFixedWidth"],
            div![
                C!["Header_gameInfoContainer"],
                attrs! {At::Title => "Flags Left"},
                view_flag_icon(C!["Header_iconSize"]),
                div![C!["Header_gameInfoText"], model.board.flags_left()]
            ]
        ],
        div![
            C!["Header_gameInfoContainer", "Header_timerSectionMargin"],
            attrs! {At::Title => "Time Elapsed"},
            view_timer_icon(C!["Header_iconSize"]),
            div![C!["Header_gameInfoText"], "3:29"]
        ]
    ]
}

fn view_action_buttons(_model: &Model) -> Node<Msg> {
    div![
        C!["Header_actionButtonsContainer"],
        div![
            C!["Header_resetButtonMargin"],
            attrs! {At::Title => "New Game"},
            view_reset_icon(C!["Header_iconSize", "Header_actionButtonImage"])
        ],
        div![
            attrs! {At::Title => "Show Hint"},
            view_hint_icon(C!["Header_iconSize", "Header_actionButtonImage"])
        ]
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
