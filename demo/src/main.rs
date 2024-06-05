use board::Board;
use yew::prelude::*;
use yew_merging_board::*;

#[function_component]
fn App() -> Html {
    html! {
        <Board style="max-width: 200px;" class={"container"} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
