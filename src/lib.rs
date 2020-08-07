mod game;
mod header;
mod icons;
mod tile;
mod update;

use game::view_game;
use seed::prelude::*;
use update::update;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board: Board::Medium(Box::new(
            [Tile {
                hidden_state: HiddenState::Safe,
                visible_state: VisibleState::Covered,
            }; 18 * 14],
        )),
        select_difficulty_element: Default::default(),
    }
}

struct Model {
    board: Board,
    select_difficulty_element: ElRef<web_sys::HtmlSelectElement>,
}

enum Board {
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
    DifficultyChange(fn(i32) -> Board),
    TileLeftClick(usize),
    TileRightClick(usize),
}

impl Board {
    fn get_tiles(&self) -> &[Tile] {
        match self {
            Board::Easy(a) => a.as_ref(),
            Board::Medium(a) => a.as_ref(),
            Board::Hard(a) => a.as_ref(),
        }
    }

    fn get_tiles_mut(&mut self) -> &mut [Tile] {
        match self {
            Board::Easy(a) => a.as_mut(),
            Board::Medium(a) => a.as_mut(),
            Board::Hard(a) => a.as_mut(),
        }
    }

    fn flags_left(&self) -> usize {
        let max_flags = match self {
            Board::Easy(_) => 10,
            Board::Medium(_) => 40,
            Board::Hard(_) => 99,
        };
        let num_flags_placed = self
            .get_tiles()
            .iter()
            .filter(|t| matches!(t.visible_state, VisibleState::Flagged))
            .count();
        max_flags - num_flags_placed
    }

    fn width(&self) -> usize {
        match self {
            Board::Easy(_) => 10,
            Board::Medium(_) => 18,
            Board::Hard(_) => 24,
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view_game);
}
