use {
  crate::{rect, TILE_AND_GAP, TILE_SIZE},
  sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window},
  std::cmp::{max, min},
};

#[derive(Debug)]
pub struct Cursor {
  pub min_x: usize,
  pub min_y: usize,
  pub max_x: usize,
  pub max_y: usize,
  pub x: usize,
  pub y: usize,
}

impl Cursor {
  pub fn left(&mut self) {
    self.x = max(self.min_x, self.x.saturating_sub(1));
  }

  pub fn right(&mut self) {
    self.x = min(self.max_x, self.x + 1);
  }

  pub fn up(&mut self) {
    self.y = max(self.min_y, self.y.saturating_sub(1));
  }

  pub fn down(&mut self) {
    self.y = min(self.max_y, self.y + 1);
  }

  pub fn draw(&self, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(240, 240, 240));

    let _ = canvas.fill_rect(rect!(self.x * TILE_AND_GAP, self.y * TILE_AND_GAP, 16, 8));
    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP,
      self.y * TILE_AND_GAP + 8,
      8,
      8
    ));

    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP + TILE_SIZE - 16,
      self.y * TILE_AND_GAP,
      16,
      8
    ));
    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP + TILE_SIZE - 8,
      self.y * TILE_AND_GAP + 8,
      8,
      8
    ));

    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP,
      self.y * TILE_AND_GAP + TILE_SIZE - 8,
      16,
      8
    ));
    let _ = canvas.fill_rect(rect!(self.x * TILE_AND_GAP, self.y * TILE_AND_GAP + TILE_SIZE - 16, 8, 8));

    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP + TILE_SIZE - 16,
      self.y * TILE_AND_GAP + TILE_SIZE - 8,
      16,
      8
    ));
    let _ = canvas.fill_rect(rect!(
      self.x * TILE_AND_GAP + TILE_SIZE - 8,
      self.y * TILE_AND_GAP + TILE_SIZE - 16,
      8,
      8
    ));
  }
}
