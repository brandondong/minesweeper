mod game;
mod header;
mod tile;

use game::view_game;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        difficulty: Difficulty::Medium(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 18 * 14],
        )),
        select_difficulty_element: Default::default(),
    }
}

struct Model {
    difficulty: Difficulty,
    select_difficulty_element: ElRef<web_sys::HtmlSelectElement>,
}

enum Difficulty {
    Easy(Box<[Tile; 10 * 8]>),
    Medium(Box<[Tile; 18 * 14]>),
    Hard(Box<[Tile; 24 * 20]>),
}

#[derive(Clone, Copy)]
struct Tile {
    visible_state: VisibleState,
    hidden_state: HiddenState,
}

#[derive(Clone, Copy)]
enum VisibleState {
    Covered,
    Uncovered,
    Flagged,
}

#[derive(Clone, Copy)]
enum HiddenState {
    Safe,
    Mine,
}

enum Msg {
    DifficultyChange(fn(i32) -> Difficulty),
    TileLeftClick(usize),
    TileRightClick(usize),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DifficultyChange(mapping) => {
            if let Some(elem) = model.select_difficulty_element.get() {
                model.difficulty = mapping(elem.selected_index());
            }
        }
        Msg::TileLeftClick(index) => {
            if let Some(tile) = get_board_mut(model).get_mut(index) {
                if let VisibleState::Covered = tile.visible_state {
                    tile.visible_state = VisibleState::Uncovered
                }
            }
        }
        Msg::TileRightClick(index) => {
            if let Some(tile) = get_board_mut(model).get_mut(index) {
                match tile.visible_state {
                    VisibleState::Covered => tile.visible_state = VisibleState::Flagged,
                    VisibleState::Flagged => tile.visible_state = VisibleState::Covered,
                    VisibleState::Uncovered => (),
                }
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![C!["App_maxCenteredContainer"], view_game(model)]
}

fn get_board(model: &Model) -> &[Tile] {
    match &model.difficulty {
        Difficulty::Easy(a) => a.as_ref(),
        Difficulty::Medium(a) => a.as_ref(),
        Difficulty::Hard(a) => a.as_ref(),
    }
}

fn get_board_mut(model: &mut Model) -> &mut [Tile] {
    match &mut model.difficulty {
        Difficulty::Easy(a) => a.as_mut(),
        Difficulty::Medium(a) => a.as_mut(),
        Difficulty::Hard(a) => a.as_mut(),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
