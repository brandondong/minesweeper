use crate::{Model, Msg, VisibleState};
use seed::prelude::*;

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DifficultyChange(mapping) => {
            if let Some(elem) = model.select_difficulty_element.get() {
                model.board = mapping(elem.selected_index());
            }
        }
        Msg::TileLeftClick(index) => {
            if let Some(tile) = model.board.get_tiles_mut().get_mut(index) {
                if let VisibleState::Covered = tile.visible_state {
                    tile.visible_state = VisibleState::Uncovered
                }
            }
        }
        Msg::TileRightClick(index) => {
            let flags_left = model.board.flags_left();
            if let Some(tile) = model.board.get_tiles_mut().get_mut(index) {
                match tile.visible_state {
                    VisibleState::Covered => {
                        if flags_left > 0 {
                            tile.visible_state = VisibleState::Flagged;
                        }
                    }
                    VisibleState::Flagged => tile.visible_state = VisibleState::Covered,
                    VisibleState::Uncovered => (),
                }
            }
        }
    }
}
