use battleship_bot::*;
use yew::prelude::*;

mod game_options;
use game_options::GameOptions;

mod function;
use function::Function;

mod game;
use game::Game;

mod options_button;
use options_button::OptionButton;

pub struct App {
    player1_place_fn: Function,
    player2_place_fn: Function,

    player1_shoot_fn: Function,
    player2_shoot_fn: Function,

    recording: Option<Recording>,
    
    show_options: bool,
}

pub enum Msg {
    ShowOptions(bool),
    SetFunction(Function, Player),
    PlayGame(Box<Recording>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            player1_place_fn: Function::PlaceFunction(place::random),
            player2_place_fn: Function::PlaceFunction(place::random),

            player1_shoot_fn: Function::ShootFunction(shoot::random),
            player2_shoot_fn: Function::ShootFunction(shoot::random),
            
            recording: None,

            show_options: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowOptions(show) => self.show_options = show,
            Msg::SetFunction(func, player) => match player {
                Player::P1 => match func {
                    Function::PlaceFunction(func) => self.player1_place_fn = Function::PlaceFunction(func),
                    Function::ShootFunction(func) => self.player1_shoot_fn = Function::ShootFunction(func),
                }
                Player::P2 => match func {
                    Function::PlaceFunction(func) => self.player2_place_fn = Function::PlaceFunction(func),
                    Function::ShootFunction(func) => self.player2_shoot_fn = Function::ShootFunction(func),
                }
            }
            Msg::PlayGame(recording) => self.recording = Some(recording.as_ref().clone())
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let options = if self.show_options {
            html!(
                <GameOptions
                    player1_fns={ (self.player1_place_fn.clone(), self.player1_shoot_fn.clone()) }
                    player2_fns={ (self.player2_place_fn.clone(), self.player2_shoot_fn.clone()) }
                    on_click={ctx.link().callback(|args: (Function, Player)| Msg::SetFunction(args.0, args.1))}
                />
            )
        } else {
            html!()
        };

        html! {
        <div>
            <OptionButton
                show_options={self.show_options}
                option_callback={ctx.link().callback(Msg::ShowOptions)}
            />
            {options}
            <Game
                player1_fns={ (self.player1_place_fn.clone(), self.player1_shoot_fn.clone()) }
                player2_fns={ (self.player2_place_fn.clone(), self.player2_shoot_fn.clone()) }
                onclick={ctx.link().callback(|recording: Recording| Msg::PlayGame(Box::new(recording)))}
                recording={self.recording.clone()}
            />
        </div>
        }
    }
}