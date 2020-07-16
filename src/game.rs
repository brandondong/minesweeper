use crate::header::view_header;
use crate::tile::view_tile;
use crate::{get_board, Difficulty, Model, Msg};
use seed::{prelude::*, *};

pub(crate) fn view_game(model: &Model) -> Node<Msg> {
    let (width_class, ratio_class, tile_class) = match model.difficulty {
        Difficulty::Easy(_) => ("Game_easyWidth", "Game_easyHeightRatio", "Game_easyTile"),
        Difficulty::Medium(_) => (
            "Game_mediumWidth",
            "Game_mediumHeightRatio",
            "Game_mediumTile",
        ),
        Difficulty::Hard(_) => ("Game_hardWidth", "Game_hardHeightRatio", "Game_hardTile"),
    };
    let tiles = get_board(model).iter().enumerate().map(|(i, t)| {
        div![
            C![tile_class],
            ev(Ev::Click, move |_| { Msg::TileLeftClick(i) }),
            ev(Ev::ContextMenu, move |event| {
                event.prevent_default();
                Msg::TileRightClick(i)
            }),
            view_tile(t)
        ]
    });
    div![
        C![width_class],
        view_header(model),
        div![C![ratio_class], div![C!["Game_boardContainer"], tiles,]]
    ]
}
