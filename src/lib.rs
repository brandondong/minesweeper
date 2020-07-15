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
    Easy([i32; 10 * 8]),
    Medium([i32; 18 * 14]),
    Hard([i32; 24 * 20]),
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Medium([0; 18 * 14])
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
        Difficulty::Easy(_) => ("Game_easyWidth", "Game_easyHeightRatio", "Game_easyTile"),
        Difficulty::Medium(_) => (
            "Game_mediumWidth",
            "Game_mediumHeightRatio",
            "Game_mediumTile",
        ),
        Difficulty::Hard(_) => ("Game_hardWidth", "Game_hardHeightRatio", "Game_hardTile"),
    };
    let tiles = get_board(model)
        .iter()
        .map(|i| div![C![tile_class], view_tile(i)]);
    div![
        C![width_class],
        view_header(model),
        div![C![ratio_class], div![C!["Game_boardContainer"], tiles,]]
    ]
}

fn view_header(model: &Model) -> Node<Msg> {
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

fn view_tile(model: &i32) -> Node<Msg> {
    div![C!["Tile_container"], model]
}

fn header_select_mapping(index: i32) -> Difficulty {
    match index {
        0 => Difficulty::Easy([0; 10 * 8]),
        1 => Difficulty::Medium([0; 18 * 14]),
        _ => Difficulty::Hard([0; 24 * 20]),
    }
}

fn get_board(model: &Model) -> &[i32] {
    match &model.difficulty {
        Difficulty::Easy(a) => a,
        Difficulty::Medium(a) => a,
        Difficulty::Hard(a) => a,
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
