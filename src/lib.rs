#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Default)]
struct Model {
    difficulty: Difficulty,
    select_difficulty_element: ElRef<web_sys::HtmlSelectElement>,
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Medium
    }
}

enum Msg {
    DifficultyChange(fn(i32) -> Difficulty),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DifficultyChange(mapping) => {
            if let Some(elem) = model.select_difficulty_element.get() {
                model.difficulty = mapping(elem.selected_index());
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![C!["App_maxCenteredContainer"], view_game(model)]
}

fn view_game(model: &Model) -> Node<Msg> {
    let (width_class, ratio_class, tile_class) = match model.difficulty {
        Difficulty::Easy => ("Game_easyWidth", "Game_easyHeightRatio", "Game_easyTile"),
        Difficulty::Medium => (
            "Game_mediumWidth",
            "Game_mediumHeightRatio",
            "Game_mediumTile",
        ),
        Difficulty::Hard => ("Game_hardWidth", "Game_hardHeightRatio", "Game_hardTile"),
    };
    div![
        C![width_class],
        view_header(model),
        div![
            C![ratio_class],
            div![C!["Game_boardContainer"], div![C![tile_class], "A"],]
        ]
    ]
}

fn view_header(model: &Model) -> Node<Msg> {
    select![
        el_ref(&model.select_difficulty_element),
        ev(Ev::Change, |_| {
            Msg::DifficultyChange(header_select_mapping)
        }),
        option![
            attrs! {At::Selected => matches!(model.difficulty, Difficulty::Easy).as_at_value()},
            "Easy"
        ],
        option![
            attrs! {At::Selected => matches!(model.difficulty, Difficulty::Medium).as_at_value()},
            "Medium"
        ],
        option![
            attrs! {At::Selected => matches!(model.difficulty, Difficulty::Hard).as_at_value()},
            "Hard"
        ]
    ]
}

fn header_select_mapping(index: i32) -> Difficulty {
    match index {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        _ => Difficulty::Hard,
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
