use {
  crate::{team::Team, UNIT_SIZE},
  sdl2::{rect::Rect, render::Canvas, video::Window},
};

pub struct Unit {
  pub x: u16,
  pub team: Team,
}

impl Unit {
  pub fn draw(&self, canvas: &mut Canvas<Window>, active: bool) {
    self.team.set_canvas_color(canvas, active);
    let _ = canvas.fill_rect(Rect::new(
      i32::from((64 + 1) * self.x) + 8,
      8,
      UNIT_SIZE,
      UNIT_SIZE,
    ));
  }
}
