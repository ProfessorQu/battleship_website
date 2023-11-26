use yew::prelude::*;
use battleship_bot::*;

use crate::function::Function;

#[derive(PartialEq, Properties)]
pub struct GameProps {
    pub player1_fns: (Function, Function),
    pub player2_fns: (Function, Function),
    pub onclick: Callback<Recording>,
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
        onclick, recording
    }: &GameProps
) -> Html {
    let game = Battleship::new(
        player1_fns.0.clone().try_into().unwrap(),
        player2_fns.0.clone().try_into().unwrap(),

        player1_fns.1.clone().try_into().unwrap(),
        player2_fns.1.clone().try_into().unwrap(),
    );

    let onclick = {
        let onclick = onclick.clone();
        Callback::from(move |_| {
            onclick.emit(game.clone().play_and_record_game())
        })
    };

    let play_button = html!( 
        <button class="important" {onclick}>{"Play game"}</button>
    );

    if recording.is_none() {
        return play_button
    }

    let recording = recording.clone().unwrap();

    html! {
    <>
        {play_button}

        <h3>{recording.clone().winner.to_string()}{" won"}</h3>
        <p>{"P1(shoot="}{player1_fns.0.name().to_lowercase()}{",place="}{player1_fns.1.name().to_lowercase()}{")"}<br />
        {"P2(shoot="}{player2_fns.0.name().to_lowercase()}{",place="}{player2_fns.1.name().to_lowercase()}{")"}</p>
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