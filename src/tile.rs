use {
  crate::TILE_SIZE,
  sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window},
};

pub struct Tile {
  pub x: u16,
}

impl Tile {
  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(60, 160, 40));
    let _ = canvas.fill_rect(Rect::new(
      i32::from((64 + 1) * self.x),
      0,
      TILE_SIZE,
      TILE_SIZE,
    ));
  }
}
