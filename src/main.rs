use yew::prelude::*;

mod game_options;
use game_options::{GameOptions, ShootFunction, Function};

mod game;
use game::generate_boats;

use crate::{game_options::{PlaceFunction, generate_table}, game::generate_shots};
use battleship_bot::*;

fn play_callback(
    game_options: UseStateHandle<GameOptions>,
    result: UseStateHandle<Option<Recording>>
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let result = result.clone();

        let mut game = Battleship::new(
            game_options.player1_place_fn.battleship_fn(),
            game_options.player2_place_fn.battleship_fn(),

            game_options.player1_shoot_fn.battleship_fn(),
            game_options.player2_shoot_fn.battleship_fn(),
        );

        result.set(Some(game.play_and_record_game()));
    })
}

#[function_component(App)]
fn app() -> Html {
    let game_options = use_state(|| GameOptions {
        player1_shoot_fn: ShootFunction::Random,
        player2_shoot_fn: ShootFunction::Random,

        player1_place_fn: PlaceFunction::Random,
        player2_place_fn: PlaceFunction::Random,
    });

    let recording = use_state(||
        None::<Recording>
    );

    let mut win_text = "No games played";

    if let Some(recording) = recording.clone().as_ref() {
        win_text = match recording.winner {
            Player::P1 => "Player 1 won",
            Player::P2 => "Player 2 won",
        };
    }

    let boats = generate_boats(recording.clone());
    let shots = generate_shots(recording.clone());

    html!(
        <>
            <h2>{ "Shoot function" }</h2>
            { generate_table(game_options.clone(), ShootFunction::list().iter().cloned()) }
            <h2>{ "Place function" }</h2>
            { generate_table(game_options.clone(), PlaceFunction::list().iter().cloned()) }
            <br />
            { "Player 1:" }<br />
            <span class="tab" />{ "Shoot: " }{ game_options.player1_shoot_fn.name() }<br />
            <span class="tab" />{ "Place: " }{ game_options.player1_place_fn.name() }<br />
            { "Player 2:"}<br />
            <span class="tab" />{ "Shoot: " }{ game_options.player2_shoot_fn.name() }<br />
            <span class="tab" />{ "Place: " }{ game_options.player2_place_fn.name() }<br />

            <button onclick={play_callback(game_options.clone(), recording.clone())}>{ "Play" }</button>
            <br />
            { win_text }
            <br />
            { boats }
            <br />
            { shots }
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}