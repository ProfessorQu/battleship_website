use battleship_bot::*;
use yew::prelude::*;

mod game_options;
use game_options::GameOptions;

mod function;
use function::Function;

mod game;
use game::Game;

pub struct App {
    player1_place_fn: Function,
    player2_place_fn: Function,

    player1_shoot_fn: Function,
    player2_shoot_fn: Function,
}

pub enum Msg {
    SetFunction(Function, Player)
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            <GameOptions player1_fns={
                    (self.player1_place_fn.clone(), self.player1_shoot_fn.clone())
                }
                player2_fns={
                    (self.player2_place_fn.clone(), self.player2_shoot_fn.clone())
                }
                on_click={ctx.link().callback(|args: (Function, Player)| Msg::SetFunction(args.0, args.1))}
                />
            <Game player1_fns={
                    (self.player1_place_fn.clone(), self.player1_shoot_fn.clone())
                }
                player2_fns={
                    (self.player2_place_fn.clone(), self.player2_shoot_fn.clone())
                }
                />
        </>
        }
    }
}