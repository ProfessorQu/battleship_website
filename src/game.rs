use battleship_bot::*;
use yew::prelude::*;

pub fn generate_boats(recording: UseStateHandle<Option<Recording>>) -> Html {
    if recording.is_none() {
        return html!()
    }

    let recording = recording.as_ref().expect("How");

    let player1_boats: Html = recording.player1_boats.iter().map(|row| {
        let row: Html = row.iter().map(|item| {
            html!(<td class={ "boat".to_string() + &(*item as usize).to_string() }></td>)
        }).collect();

        html!(
            <tr>{row}</tr>
        )
    }).collect();

    let player2_boats: Html = recording.player2_boats.iter().map(|row| {
        let row: Html = row.iter().map(|item| {
            html!(<td class={ "boat".to_string() + &(*item as usize).to_string() }></td>)
        }).collect();

        html!(
            <tr>{row}</tr>
        )
    }).collect();

    html!(
        <>
            { "Boats" }
            <br />
            <table class="boats">
                <tr>{ "P1" }</tr>
                { player1_boats }
            </table>
            <span class="tab" />
            <table class="boats">
                <tr>{ "P2" }</tr>
                { player2_boats }
            </table>
        </>
    )
}