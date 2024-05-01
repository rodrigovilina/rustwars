#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

mod back;
mod cursor;
mod front;
mod team;
mod tile;
mod turn;
mod unit;

use {
  ::std::thread::sleep,
  back::Backend,
  cursor::Cursor,
  front::Frontend,
  sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, ttf::Font},
  std::{path::Path, time::Duration},
  unit::View,
};

#[macro_export]
macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

pub const TILE_SIZE: usize = 64;
pub const TILE_AND_GAP: usize = 65;
pub const UNIT_SIZE: usize = 48;

fn main() -> Result<(), String> {
  Game::main()
}

pub struct Game {
  front: Frontend,
  back: Backend,
}

impl Game {
  fn main() -> Result<(), String> {
    Self::new()?.run()
  }

  fn new() -> Result<Self, String> {
    let back = Backend::new();
    let front = Frontend::new()?;

    Ok(Self { front, back })
  }

  fn run(&mut self) -> Result<(), String> {
    let mut cursor = Cursor {
      min_x: 0,
      min_y: 0,
      max_x: 2,
      max_y: 2,
      x: 0,
      y: 0,
    };

    self.front.clear_and_present_black();

    let mut change_turn: bool;

    loop {
      change_turn = false;

      for event in self.front.event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => return Ok(()),
          Event::KeyDown {
            keycode: Some(Keycode::T),
            ..
          } => change_turn = true,
          Event::KeyDown {
            keycode: Some(Keycode::J),
            ..
          } => cursor.left(),
          Event::KeyDown {
            keycode: Some(Keycode::K),
            ..
          } => cursor.down(),
          Event::KeyDown {
            keycode: Some(Keycode::I),
            ..
          } => cursor.up(),
          Event::KeyDown {
            keycode: Some(Keycode::L),
            ..
          } => cursor.right(),
          _ => {}
        }
      }

      self.clear_black();
      self.draw_tiles();
      self.draw_units();
      self.draw_cursor(&cursor);
      self.draw_turn_bar();
      self.draw_text(&cursor)?;
      self.present();
      self.end_turn(change_turn);
      self.increase_frame_count();
      sleep(Duration::from_millis(50));
    }
  }

  fn end_turn(&mut self, change_turn: bool) {
    if change_turn {
        self.back.turn = self.back.turn.next();
      }
  }

  fn increase_frame_count(&mut self) {
    self.front.frame += 1;
  }

  fn present(&mut self) {
    self.front.canvas.present();
  }

  fn clear_black(&mut self) {
    self.front.canvas.set_draw_color(Color::RGB(20, 20, 20));
    self.front.canvas.clear();
  }

  fn draw_text(&mut self, cursor: &Cursor) -> Result<(), String> {
    let font_path = Path::new("./font.ttf");
    let font: Font<'_, '_> = self.front.ttf_context.load_font(font_path, 32)?;
    let surface = font
      .render(&format!(
        "Hello Rust! frame: {}, Current turn: {:?}",
        self.front.frame, self.back.turn.team
      ))
      .blended(Color::RGBA(255, 0, 0, 255))
      .map_err(|e| e.to_string())?;
    let texture = self
      .front
      .texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;
    let target = rect!(0, 80, 800, 80);
    self.front.canvas.copy(&texture, None, Some(target))?;
    let surface = font
      .render(&format!("Cursor: {cursor:?}",))
      .blended(Color::RGBA(255, 0, 0, 255))
      .map_err(|e| e.to_string())?;
    let texture = self
      .front
      .texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;
    let target = rect!(0, 160, 800, 80);
    self.front.canvas.copy(&texture, None, Some(target))?;
    Ok(())
  }

  fn draw_cursor(&mut self, cursor: &Cursor) {
    cursor.draw(&mut self.front.canvas);
  }

  fn draw_units(&mut self) {
    self.back.units.iter().for_each(|unit| {
      let active: bool = unit.team == self.back.turn.team;
      let unit_view: View = if active {
        View::Active(unit.clone())
      } else {
        View::Inactive(unit.clone())
      };
      unit_view.draw(&mut self.front.canvas);
    });
  }

  fn draw_tiles(&mut self) {
    self.back.tiles.iter().for_each(|tile| {
      tile.draw(&mut self.front.canvas);
    });
  }

  fn draw_turn_bar(&mut self) {
    self
      .front
      .canvas
      .set_draw_color(self.back.turn.team.light_color());
    let _ = self.front.canvas.fill_rect(Rect::new(0, 590, 800, 10));
  }
}
