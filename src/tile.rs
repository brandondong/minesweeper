use crate::icons::view_flag_icon;
use crate::{HiddenState, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_tile(tile: &Tile) -> Node<Msg> {
    let top_layer = match tile.visible_state {
        VisibleState::Covered => div![
            C!["Tile_coveredBackground"],
            view_flag_icon(C!["Tile_flagIcon", "Tile_flagIconHidden"])
        ],
        VisibleState::Flagged => div![
            C!["Tile_coveredBackground"],
            view_flag_icon(C!["Tile_flagIcon"])
        ],
        VisibleState::Uncovered => {
            div![C!["Tile_coveredBackground", "Tile_coveredBackgroundHidden"]]
        }
    };
    let bottom_layer = match tile.hidden_state {
        HiddenState::Safe => "s",
        HiddenState::Mine => "m",
    };
    div![
        C!["Tile_container"],
        div![C!["Tile_topLayer"], top_layer],
        bottom_layer
    ]
}
