use {
  crate::{rect, team::Team, TILE_AND_GAP, UNIT_SIZE},
  sdl2::{rect::Rect, render::Canvas, video::Window},
};

#[derive(Clone)]
pub struct Unit {
  pub x: u16,
  pub y: u16,
  pub hp: usize,
  pub team: Team,
}

#[derive(Clone)]
pub enum View {
  Active(Unit),
  Inactive(Unit),
}

impl View {
  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    match self {
      Self::Active(unit) => {
        unit.team.set_canvas_color(canvas, true);
        let _ = canvas.fill_rect(rect!(
          TILE_AND_GAP * unit.x as usize + 8,
          TILE_AND_GAP * unit.y as usize + 8,
          UNIT_SIZE,
          UNIT_SIZE
        ));
      }
      Self::Inactive(unit) => {
        unit.team.set_canvas_color(canvas, false);
        let _ = canvas.fill_rect(rect!(
          TILE_AND_GAP * unit.x as usize + 8,
          TILE_AND_GAP * unit.y as usize + 8,
          UNIT_SIZE,
          UNIT_SIZE
        ));
      }
    }
  }
}
