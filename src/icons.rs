use crate::Msg;
use seed::{prelude::*, *};

pub(crate) fn view_flag_icon(attrs: Attrs) -> Node<Msg> {
    svg![
        attrs,
        attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Stroke => "#FF5722", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
        path![
            C!["Icon_flag"],
            attrs! {At::D => "M5 5a5 5 0 0 1 7 0a5 5 0 0 0 7 0 L 19 14 a5 5 0 0 1 -7 0a5 5 0 0 0 -7 0", At::Fill => "#FF5722"}
        ],
        line_![attrs! {At::X1 => "5", At::Y1 => "5", At::X2 => "5", At::Y2 => "21"}]
    ]
}

pub(crate) fn view_timer_icon(attrs: Attrs) -> Node<Msg> {
    svg![
        attrs,
        attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Stroke => "#FFFFFF", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
        circle![attrs! {At::Cx => "12", At::Cy => "12", At::R => "9"}],
        polyline![attrs! {At::Points => "12 7 12 12 15 15"}]
    ]
}

pub(crate) fn view_reset_icon(attrs: Attrs) -> Node<Msg> {
    svg![
        attrs,
        attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Stroke => "#FFFFFF", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
        path![attrs! {At::D => "M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -5v5h5"}],
        path![attrs! {At::D => "M4 13a8.1 8.1 0 0 0 15.5 2m.5 5v-5h-5"}]
    ]
}

pub(crate) fn view_hint_icon(attrs: Attrs) -> Node<Msg> {
    svg![
        attrs,
        attrs! {At::ViewBox => "0 0 24 24", At::StrokeWidth => "1.5", At::Stroke => "#FFFFFF", At::Fill => "none", At::StrokeLinecap => "round", At::StrokeLineJoin => "round"},
        path![attrs! {At::D => "M3 12h1M12 3v1M20 12h1M5.6 5.6l.7 .7M18.4 5.6l-.7 .7"}],
        path![
            attrs! {At::D => "M9 16a5 5 0 1 1 6 0a3.5 3.5 0 0 0 -1 3a2 2 0 0 1 -4 0a3.5 3.5 0 0 0 -1 -3" }
        ],
        line_![attrs! {At::X1 => "9.7", At::Y1 => "17", At::X2 => "14.3", At::Y2 => "17"}]
    ]
}
