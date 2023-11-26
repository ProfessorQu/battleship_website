use battleship_bot::*;

#[derive(PartialEq, Clone)]
pub enum Function {
    ShootFunction(fn(Pos, ShotMap) -> Pos),
    PlaceFunction(fn() -> BoatMap)
}

impl Function {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ShootFunction(func) => {
                if *func == shoot::random as fn(Pos, ShotMap) -> Pos {
                    "Random"
                } else if *func == shoot::random_and_random_destroy as fn(Pos, ShotMap) -> Pos {
                    "Random and random destroy"
                } else if *func == shoot::random_and_destroy as fn(Pos, ShotMap) -> Pos {
                    "Random and destroy"
                } else if *func == shoot::grid_and_destroy as fn(Pos, ShotMap) -> Pos {
                    "Grid and destroy"
                } else if *func == shoot::heatmap_and_destroy as fn(Pos, ShotMap) -> Pos {
                    "Heatmap and destroy"
                } else {
                    "Unkwown"
                }
            },
            Self::PlaceFunction(func) => {
                if *func == place::random as fn() -> BoatMap {
                    "Random"
                } else if *func == place::sides as fn() -> BoatMap {
                    "Sides"
                } else if *func == place::spread as fn() -> BoatMap {
                    "Spread"
                } else if *func == place::cluster as fn() -> BoatMap {
                    "Cluster"
                } else {
                    "Unkwown"
                }
            }
        }
    }

    pub fn list(place: bool) -> Vec<Function> {
        if place {
            vec![
                Function::PlaceFunction(place::random),
                Function::PlaceFunction(place::sides),
                Function::PlaceFunction(place::spread),
                Function::PlaceFunction(place::cluster)
            ]
        } else {
            vec![
                Function::ShootFunction(shoot::random),
                Function::ShootFunction(shoot::random_and_random_destroy),
                Function::ShootFunction(shoot::random_and_destroy),
                Function::ShootFunction(shoot::grid_and_destroy),
                Function::ShootFunction(shoot::heatmap_and_destroy),
            ]
        }
    }
}

impl TryInto<fn() -> BoatMap> for Function {
    type Error = ();

    fn try_into(self) -> Result<fn() -> BoatMap, Self::Error> {
        match self {
            Self::PlaceFunction(func) => Ok(func),
            Self::ShootFunction(_) => Err(())
        }
    }
}

impl TryInto<fn(Pos, ShotMap) -> Pos> for Function {
    type Error = ();

    fn try_into(self) -> Result<fn(Pos, ShotMap) -> Pos, Self::Error> {
        match self {
            Self::ShootFunction(func) => Ok(func),
            Self::PlaceFunction(_) => Err(())
        }
    }
}
