use crate::{HiddenState, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_tile(tile: &Tile) -> Node<Msg> {
    let s = match (tile.visible_state, tile.hidden_state) {
        (VisibleState::Covered, _) => "c",
        (VisibleState::Flagged, _) => "f",
        (VisibleState::Uncovered, HiddenState::Safe) => "s",
        (VisibleState::Uncovered, HiddenState::Mine) => "m",
    };
    div![C!["Tile_container"], s]
}
