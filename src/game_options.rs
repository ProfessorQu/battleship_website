use battleship_bot::*;
use yew::prelude::*;

use crate::Function;

#[derive(PartialEq, Properties)]
pub struct GameOptionsProps {
    pub player1_fns: (Function, Function),
    pub player2_fns: (Function, Function),
    pub on_click: Callback<(Function, Player)>
}

fn generate_buttons(on_click: &Callback<(Function, Player)>, player: Player, place: bool) -> Html {
    Function::list(place).iter().map(|func| {
        let click = {
            let on_click = on_click.clone();
            let func = func.clone();
            Callback::from(move |_| {
                on_click.emit((func.clone(), player))
            })
        };

        html!(
            <button onclick={click}>{func.name()}</button>
        )
    }).collect()
}

#[function_component]
pub fn GameOptions(
    GameOptionsProps { player1_fns, player2_fns, on_click }: &GameOptionsProps
) -> Html {
    html! {
        <>
            <h2>{"Player 1"}</h2>
            {"Place: "}{player1_fns.0.name()}<br />
            {"Shoot: "}{player1_fns.1.name()}<br />
            {generate_buttons(on_click, Player::P1, true)}<br />
            {generate_buttons(on_click, Player::P1, false)}

            <h2>{"Player 2"}</h2>
            {"Place: "}{player2_fns.0.name()}<br />
            {"Shoot: "}{player2_fns.1.name()}<br />
            {generate_buttons(on_click, Player::P2, true)}<br />
            {generate_buttons(on_click, Player::P2, false)}<br />
        </>
    }
}