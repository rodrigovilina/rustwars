use crate::{
  team::Team,
  tile::{Tile, Type},
  turn::Turn, unit::Unit,
};

pub struct Backend {
  pub turn: Turn,
  pub tiles: Vec<Tile>,
  pub units: Vec<Unit>,
}

impl Backend {
  pub fn new() -> Self {
    let turn = Turn { team: Team::Red };
    let tiles: Vec<Tile> = vec![
      Tile {
        x: 0,
        y: 0,
        r#type: Type::Ocean,
      },
      Tile {
        x: 1,
        y: 0,
        r#type: Type::Ocean,
      },
      Tile {
        x: 2,
        y: 0,
        r#type: Type::Ocean,
      },
      Tile {
        x: 0,
        y: 1,
        r#type: Type::Ocean,
      },
      Tile {
        x: 1,
        y: 1,
        r#type: Type::Ocean,
      },
      Tile {
        x: 2,
        y: 1,
        r#type: Type::Grass,
      },
      Tile {
        x: 0,
        y: 2,
        r#type: Type::Ocean,
      },
      Tile {
        x: 1,
        y: 2,
        r#type: Type::Grass,
      },
      Tile {
        x: 2,
        y: 2,
        r#type: Type::Grass,
      },
    ];
    let units = vec![
      Unit {
        x: 1,
        y: 2,
        hp: 10,
        team: Team::Red,
      },
      Unit {
        x: 2,
        y: 2,
        hp: 10,
        team: Team::Blue,
      }
    ];

    Self { turn, tiles, units }
  }
}
