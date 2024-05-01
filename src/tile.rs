use {
  crate::{rect, TILE_AND_GAP, TILE_SIZE},
  sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window},
};

pub enum Type {
  Grass,
  Ocean,
}

pub struct Tile {
  pub x: u16,
  pub y: u16,
  pub r#type: Type,
}

impl Tile {
  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    let color = self.r#type.color();
    canvas.set_draw_color(color);
    let _ = canvas.fill_rect(rect!(
      TILE_AND_GAP * self.x as usize,
      TILE_AND_GAP * self.y as usize,
      TILE_SIZE,
      TILE_SIZE
    ));
  }
}

impl Type {
  pub const fn color(&self) -> Color {
    match self {
      Self::Grass => Color::RGB(60, 160, 40),
      Self::Ocean => Color::RGB(60, 40, 160),
    }
  }
}
