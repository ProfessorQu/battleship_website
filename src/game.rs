use std::{fmt::Display, any::Any};

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

pub struct Game {
    pub player1_shoot_fn: ShootFunction,
    pub player2_shoot_fn: ShootFunction,

    pub player1_place_fn: PlaceFunction,
    pub player2_place_fn: PlaceFunction,
}
