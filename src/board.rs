use yew::prelude::*;
use yew_hooks::use_size;

use crate::square::{File, Rank, Square};

#[derive(Properties, PartialEq, Default)]
pub struct BoardProps {
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    let BoardProps { style, class } = props;

    let wrap_node = use_node_ref();
    let size = use_size(wrap_node.clone());

    let mut pieces = vec![];
    for rank in Rank::ALL {
        for file in File::ALL {
            let square = Square::from_coords(file, rank);

            // squares start at bottom left,
            // but screen coordinates start at top left.
            // so, Y coordinate is inverted
            // Also, first rank should have value of 1 (not 0 as default),
            // otherwise the top row is empty and one extra row is used at the bottom
            let translate_x = u32::from(file) * size.0 / 8;
            let translate_y = size.0 - (u32::from(rank) + 1) * size.0 / 8;

            pieces.push(html! {
                <piece class={classes!("pawn", if square.is_light() { "white" } else { "black" })} style={format!("transform: translate({translate_x}px, {translate_y}px)")}></piece>
            })
        }
    }

    let style = format!("height: {}px !important; {style}", size.0);
    html! {
        <div class={classes!("cg-wrap", class.clone())} {style} ref={wrap_node}>
            <cg-container>
                <cg-board>
                    {pieces}
                </cg-board>
            </cg-container>
        </div>
    }
}
