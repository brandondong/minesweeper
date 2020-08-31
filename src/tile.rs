use crate::icons::view_flag_icon;
use crate::{HiddenState, Msg, Tile, VisibleState};
use seed::{prelude::*, *};

pub(crate) fn view_tile(tile: &Tile, highlighted: bool) -> Node<Msg> {
    let (top_color, bottom_color) = if highlighted {
        ("Tile_topHighlight", "Tile_bottomHighlight")
    } else {
        ("Tile_topNormal", "Tile_bottomNormal")
    };
    let top_content = match tile.visible_state {
        VisibleState::Covered => view_flag_icon(C!["Tile_flagIcon", "Tile_flagIconHidden"]),
        VisibleState::Flagged => view_flag_icon(C!["Tile_flagIcon", "Tile_flagIconVisible"]),
        VisibleState::Uncovered => empty![],
    };
    let bottom_content = match tile.hidden_state {
        HiddenState::Safe => "s",
        HiddenState::Mine => "m",
    };
    let uncovered = matches!(tile.visible_state, VisibleState::Uncovered);
    div![
        C!["Tile_container"],
        div![
            C![
                "Tile_topLayer",
                top_color,
                IF!(uncovered => "Tile_topLayerHidden"),
            ],
            top_content
        ],
        IF!(!uncovered => div![C!["Tile_topLayerBorder"]]),
        div![C!["Tile_bottomLayer", bottom_color], bottom_content]
    ]
}
