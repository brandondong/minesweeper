use crate::icons::view_flag_icon;
use crate::{HiddenState, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_tile(tile: &Tile) -> Node<Msg> {
    match (tile.visible_state, tile.hidden_state) {
        (VisibleState::Covered, _) => div![C!["Tile_coveredBackground"], "c"],
        (VisibleState::Flagged, _) => div![
            C!["Tile_coveredBackground"],
            view_flag_icon(C!["Tile_flagIcon"])
        ],
        (VisibleState::Uncovered, HiddenState::Safe) => div![C!["Tile_uncoveredBackground"], "s"],
        (VisibleState::Uncovered, HiddenState::Mine) => div![C!["Tile_uncoveredBackground"], "m"],
    }
}
