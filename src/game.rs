use battleship_bot::*;
use yew::prelude::*;

pub fn generate_boats(recording: UseStateHandle<Option<Recording>>) -> Html {
    if recording.is_none() {
        return html!()
    }

    let recording = recording.as_ref().expect("How");

    let boats: Html = recording.player1_boats.iter().map(|row| {
        let row: Html = row.iter().map(|item| {
            html!(<td class={ "boat".to_string() + &(*item as usize).to_string() }></td>)
        }).collect();

        html!(
            <tr>{row}</tr>
        )
    }).collect();

    html!(
        <table class="boats">
            { boats }
        </table>
    )
}