use yew::prelude::*;
use battleship_bot::*;

use crate::function::Function;

#[derive(PartialEq, Properties)]
pub struct GameProps {
    pub player1_fns: (Function, Function),
    pub player2_fns: (Function, Function),
    pub on_click: Callback<Recording>,
    pub recording: Option<Recording>,
}

fn generate_boats(recording: Recording, player: Player) -> Html {
    let boats = match player {
        Player::P1 => recording.player1_boats,
        Player::P2 => recording.player2_boats
    };

    let boats_html: Html = boats.iter().map(|row| {
        let row_html: Html = row.iter().map(|item| {
            if *item != Boat::Empty {
                html!(<td class={ "boat".to_string() + &(*item as usize).to_string() } />)
            } else {
                html!(<td />)
            }
        }).collect();

        html!(
            <tr>{row_html}</tr>
        )
    }).collect();

    html!(
        <table class="boats">
            <tr>{player.to_string()}</tr>
            {boats_html}
        </table>
    )
}

fn generate_shots(recording: Recording, player: Player) -> Html {
    let shots = match player {
        Player::P1 => recording.player1_shots.last().unwrap(),
        Player::P2 => recording.player2_shots.last().unwrap(),
    };

    let shots_html: Html = shots.iter().map(|row| {
        let row_html: Html = row.iter().map(|item| {
            if let Some(shot) = item {
                match shot {
                    Shot::Hit(_) => html!(<td class={"hit"} />),
                    Shot::Miss => html!(<td class={"miss"} />)
                }
            } else {
                html!(<td />)
            }
        }).collect();

        html!(
            <tr>{row_html}</tr>
        )
    }).collect();

    html!(
        <table class="shots">
            <tr>{player.to_string()}</tr>
            {shots_html}
        </table>
    )
}

#[function_component]
pub fn Game(
    GameProps {
        player1_fns, player2_fns,
        on_click, recording
    }: &GameProps
) -> Html {
    let game = Battleship::new(
        player1_fns.0.clone().try_into().unwrap(),
        player2_fns.0.clone().try_into().unwrap(),

        player1_fns.1.clone().try_into().unwrap(),
        player2_fns.1.clone().try_into().unwrap(),
    );

    let on_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(game.clone().play_and_record_game())
        })
    };

    if recording.is_none() {
        return html!( 
            <button onclick={on_click}>{"Play game"}</button>
        )
    }

    let recording = recording.clone().unwrap();

    html! {
    <>
        <button onclick={on_click}>{"Play game"}</button>

        <h2>{recording.clone().winner.to_string()}{" won"}</h2>
        {generate_boats(recording.clone(), Player::P1)}
        <span class="tab" />
        {generate_boats(recording.clone(), Player::P2)}
        <br />

        {generate_shots(recording.clone(), Player::P1)}
        <span class="tab" />
        {generate_shots(recording.clone(), Player::P2)}
    </>
    }
}