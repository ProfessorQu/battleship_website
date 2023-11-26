use battleship_bot::*;
use yew::prelude::*;

use crate::Function;

#[derive(PartialEq, Properties)]
pub struct GameOptionsProps {
    pub player1_fns: (Function, Function),
    pub player2_fns: (Function, Function),
    pub on_click: Callback<(Function, Player)>,
}

fn generate_buttons(on_click: &Callback<(Function, Player)>, player: Player, place: bool, fns: (Function, Function)) -> Html {
    Function::list(place).iter().map(|func| {
        let click = {
            let on_click = on_click.clone();
            let func = func.clone();
            Callback::from(move |_| {
                on_click.emit((func.clone(), player))
            })
        };

        let class = if place {
            if *func == fns.0 {
                "active"
            } else {
                ""
            }
        } else if *func == fns.1 {
            "active"
        } else {
            ""
        };

        html!(
            <tr><td><button onclick={click} class={class}>{func.name()}</button></td></tr>
        )
    }).collect()
}

fn player1_options(on_click: &Callback<(Function, Player)>, player1_fns: (Function, Function)) -> Html {
    html!(
        <div>
            <h2>{"Player 1"}</h2>
            <table class="options">
                <th>{"Shoot"}</th>
                {generate_buttons(on_click, Player::P1, true, player1_fns.clone())}
            </table>
            <table class="options">
                <th>{"Place"}</th>
                {generate_buttons(on_click, Player::P1, false, player1_fns.clone())}
            </table>
        </div>
    )
}

fn player2_options(on_click: &Callback<(Function, Player)>, player2_fns: (Function, Function)) -> Html {
    html!(
        <div>
            <h2>{"Player 2"}</h2>
            <table class="options">
                <th>{"Shoot"}</th>
                {generate_buttons(on_click, Player::P2, true, player2_fns.clone())}
            </table>
            <table class="options">
                <th>{"Place"}</th>
                {generate_buttons(on_click, Player::P2, false, player2_fns.clone())}
            </table>
        </div>
    )
}

#[function_component]
pub fn GameOptions(
    GameOptionsProps {
        player1_fns,
        player2_fns,
        on_click,
    }: &GameOptionsProps
) -> Html {
    html! {
        <>
            {player1_options(on_click, player1_fns.clone())}
            {player2_options(on_click, player2_fns.clone())}
        </>
    }
}