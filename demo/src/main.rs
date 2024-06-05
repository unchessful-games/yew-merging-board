use board::Board;
use yew::prelude::*;
use yew_merging_board::*;

#[function_component]
fn App() -> Html {
    html! {
        <Board style="max-width: 50%;" class={"container"} />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
