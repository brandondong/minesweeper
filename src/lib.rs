#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

type Model = i32;

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![C!["App_maxCenteredContainer"], view_game(model)]
}

fn view_game(model: &Model) -> Node<Msg> {
    div![
        C!["Game_mediumWidth"],
        view_header(model),
        div![
            C!["Game_mediumHeightRatio"],
            div![
                C!["Game_boardContainer"],
                div![
                    C!["Game_mediumTile"],
                    model,
                    ev(Ev::Click, |_| Msg::Increment),
                ],
            ]
        ]
    ]
}

fn view_header(model: &Model) -> Node<Msg> {
    div![model]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
