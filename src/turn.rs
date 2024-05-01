use crate::team::Team;

#[derive(Debug)]
pub struct Turn {
  pub team: Team,
}

impl Turn {
  pub const fn next(&self) -> Self {
    match self.team {
      Team::Red => Self { team: Team::Blue },
      Team::Blue => Self { team: Team::Red },
    }
  }
}
