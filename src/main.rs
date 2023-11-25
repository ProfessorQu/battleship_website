use yew::prelude::*;

mod game;
use game::{Game, ShootFunction, Function};

use crate::game::PlaceFunction;

fn get_callback<F>(
    game: UseStateHandle<Game>,
    func: F,
    player: usize,
) -> Callback<MouseEvent>
where F: Function    
{
    Callback::from(move |_| {
        let mut new_game = Game {
            player1_shoot_fn: game.player1_shoot_fn,
            player2_shoot_fn: game.player2_shoot_fn,

            player1_place_fn: game.player1_place_fn,
            player2_place_fn: game.player2_place_fn,
        };

        if let Some(shoot_fn) = func.as_any().downcast_ref::<ShootFunction>() {
            match player {
                1 => new_game.player1_shoot_fn = *shoot_fn,
                2 => new_game.player2_shoot_fn = *shoot_fn,
                _ => {}
            }
        } else if let Some(place_fn) = func.as_any().downcast_ref::<PlaceFunction>() {
            match player {
                1 => new_game.player1_place_fn = *place_fn,
                2 => new_game.player2_place_fn = *place_fn,
                _ => {}
            }
        }

        game.set(new_game);
    })
}

fn generate_button<F>(
    game: UseStateHandle<Game>,
    func: F,
    player: usize,
    text: &str,
) -> Html
where F: Function
{
    let callback = get_callback(game, func, player);
    html! {
        <button onclick={callback}>{ text }</button>
    }
}

fn create_table<F, I>(game: UseStateHandle<Game>, functions: I, is_shoot: bool) -> Html
    where
    F: Function + Copy,
    I: Iterator<Item = F>
{
    let headers = html!(
        <tr>
            <th>{ "Player 1" }</th>
            <th>{ "Player 2" }</th>
        </tr>
    );

    let buttons: Html = functions.map(|func| {
        html!(
            <tr>
                <td>{generate_button(game.clone(), func, 1, func.name())}</td>
                <td>{generate_button(game.clone(), func, 2, func.name())}</td>
            </tr>
        )
    }).collect();

    let data = if is_shoot {
        html!(
            <tr>
                <th><div class="func">{format!("{}", game.player1_shoot_fn)}</div></th>
                <th><div class="func">{format!("{}", game.player2_shoot_fn)}</div></th>
            </tr>
        )
    } else {
        html!(
            <tr>
                <th><div class="func">{format!("{}", game.player1_place_fn)}</div></th>
                <th><div class="func">{format!("{}", game.player2_place_fn)}</div></th>
            </tr>
        )
    };

    html!(
        <table>
            { headers }
            { buttons }
            { data }
        </table>
    )
}

#[function_component(App)]
fn app() -> Html {
    let game = use_state(|| Game {
        player1_shoot_fn: ShootFunction::Random,
        player2_shoot_fn: ShootFunction::Random,

        player1_place_fn: PlaceFunction::Random,
        player2_place_fn: PlaceFunction::Random,
    });

    html!(
        <>
        { create_table(game.clone(), ShootFunction::list().iter().cloned(), true) }
        <br />
        { create_table(game.clone(), PlaceFunction::list().iter().cloned(), false) }
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}