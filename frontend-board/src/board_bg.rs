use merging_board_logic::square::{File, Rank, Square};
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Node};
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_hooks::use_size;

#[autoprops]
#[function_component(BoardBackground)]
pub fn board_bg(children: &Children, onclick_square: Option<Callback<Square>>) -> Html {
    let node = use_node_ref();
    let size = use_size(node.clone());

    let onclick = {
        shadow_clone!(onclick_square, size, node);
        move |ev: MouseEvent| {
            // If this event isn't originally targeted at the board background,
            // don't do anything
            let target = ev.target().unwrap().dyn_into::<HtmlElement>();
            if let Ok(target) = target {
                if !target.is_same_node(node.cast::<Node>().as_ref()) {
                    return;
                }
            }
            if let Some(onclick_square) = onclick_square.clone() {
                let rel_x = (ev.offset_x() as f32) / size.0 as f32;
                let rel_y = (ev.offset_y() as f32) / size.0 as f32;
                let rel_y = 1.0 - rel_y;
                let square = Square::from_coords(
                    File::new((rel_x * 8.0) as u32),
                    Rank::new((rel_y * 8.0) as u32),
                );
                onclick_square.emit(square);
            }
        }
    };
    html! {
        <cg-board {onclick} ref={node}>
            { for children.iter() }
        </cg-board>
    }
}
