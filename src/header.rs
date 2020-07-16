use crate::{Difficulty, HiddenState, Model, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_header(model: &Model) -> Node<Msg> {
    let easy_selected = matches!(model.difficulty, Difficulty::Easy(_));
    let medium_selected = matches!(model.difficulty, Difficulty::Medium(_));
    let hard_selected = matches!(model.difficulty, Difficulty::Hard(_));
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

fn header_select_mapping(index: i32) -> Difficulty {
    match index {
        0 => Difficulty::Easy(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 10 * 8],
        )),
        1 => Difficulty::Medium(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 18 * 14],
        )),
        _ => Difficulty::Hard(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 24 * 20],
        )),
    }
}
