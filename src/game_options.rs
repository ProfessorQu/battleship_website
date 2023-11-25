use std::{fmt::Display, any::Any};

use battleship_bot::{place, Pos, ShotMap, shoot, BoatMap};
use yew::prelude::*;

pub trait Function: Any {
    fn name(&self) -> &'static str;
    fn list() -> Vec<Self>
    where
        Self: Sized;
    
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, PartialEq)]
pub enum ShootFunction {
    Random,
    RandomRandomDestroy,
    RandomDestroy,
    GridDestroy,
    HeatmapDestroy
}

impl Function for ShootFunction {
    fn list() -> Vec<ShootFunction> {
        vec![
            Self::Random,
            Self::RandomRandomDestroy,
            Self::RandomDestroy,
            Self::GridDestroy,
            Self::HeatmapDestroy
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Random => "Random",
            Self::RandomRandomDestroy => "Random and random destroy",
            Self::RandomDestroy => "Random and destroy",
            Self::GridDestroy => "Grid and destroy",
            Self::HeatmapDestroy => "Heatmap and destroy",
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShootFunction {
    pub fn battleship_fn(&self) -> fn(Pos, ShotMap) -> Pos {
        match self {
            Self::Random => shoot::random,
            Self::RandomRandomDestroy => shoot::random_and_random_destroy,
            Self::RandomDestroy => shoot::random_and_destroy,
            Self::GridDestroy => shoot::grid_and_destroy,
            Self::HeatmapDestroy => shoot::heatmap_and_destroy,
        }
    }
}

impl Display for ShootFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlaceFunction {
    Random,
    Sides,
    Spread,
    Cluster
}

impl Function for PlaceFunction {
    fn list() -> Vec<PlaceFunction> {
        vec![
            Self::Random,
            Self::Sides,
            Self::Spread,
            Self::Cluster,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Random => "Random",
            Self::Sides => "Side",
            Self::Spread => "Spread",
            Self::Cluster => "Cluster",
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PlaceFunction {
    pub fn battleship_fn(&self) -> fn() -> BoatMap {
        match self {
            Self::Random => place::random,
            Self::Sides => place::sides,
            Self::Spread => place::spread,
            Self::Cluster => place::cluster,
        }
    }
}

impl Display for PlaceFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Random => write!(f, "Random"),
            Self::Sides => write!(f, "Sides"),
            Self::Spread => write!(f, "Spread"),
            Self::Cluster => write!(f, "Cluster"),
        }
    }
}

pub struct GameOptions {
    pub player1_shoot_fn: ShootFunction,
    pub player2_shoot_fn: ShootFunction,

    pub player1_place_fn: PlaceFunction,
    pub player2_place_fn: PlaceFunction,
}

fn get_callback<F>(
    game: UseStateHandle<GameOptions>,
    func: F,
    player: usize,
) -> Callback<MouseEvent>
where F: Function    
{
    Callback::from(move |_| {
        let mut new_game = GameOptions {
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
    game: UseStateHandle<GameOptions>,
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

pub fn generate_table<F, I>(game: UseStateHandle<GameOptions>, functions: I) -> Html
    where
    F: Function + Copy,
    I: Iterator<Item = F>
{
    let headers = html!(
        <tr>
            <th class="player">{ "Player 1" }</th>
            <th class="player">{ "Player 2" }</th>
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

    html!(
        <table>
            { headers }
            { buttons }
        </table>
    )
}
