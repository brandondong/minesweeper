use crate::header::view_header;
use crate::tile::view_tile;
use crate::{Board, Model, Msg};
use seed::{prelude::*, *};

pub(crate) fn view_game(model: &Model) -> Node<Msg> {
    let (width_class, ratio_class, tile_class) = match model.board {
        Board::Easy(_) => ("Game_easyWidth", "Game_easyHeightRatio", "Game_easyTile"),
        Board::Medium(_) => (
            "Game_mediumWidth",
            "Game_mediumHeightRatio",
            "Game_mediumTile",
        ),
        Board::Hard(_) => ("Game_hardWidth", "Game_hardHeightRatio", "Game_hardTile"),
    };
    let width = model.board.width();
    let tiles = model.board.get_tiles().iter().enumerate().map(|(i, t)| {
        let highlighted = i % 2 == (i / width) % 2;
        div![
            el_key(&i),
            C![tile_class],
            ev(Ev::Click, move |_| Msg::TileLeftClick(i)),
            ev(Ev::ContextMenu, move |event| {
                event.prevent_default();
                Msg::TileRightClick(i)
            }),
            view_tile(t, highlighted)
        ]
    });
    div![
        C![width_class, "Game_minContent"],
        view_header(model),
        div![C![ratio_class], div![C!["Game_boardContainer"], tiles,]]
    ]
}
