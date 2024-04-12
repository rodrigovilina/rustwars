#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

mod tile;
mod unit;
mod team;

use {
  sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect,
  }, ::std::thread::sleep, std::{path::Path, time::Duration}, team::Team, tile::Tile, unit::Unit
};

macro_rules! rect(
  ($x:expr, $y:expr, $w:expr, $h:expr) => (
    Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
  )
);

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

  let window = video_subsystem
    .window("rust-sdl2 demo: Video", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  let texture_creator = canvas.texture_creator();
  let font_path = Path::new("./font.ttf");
  let font = ttf_context.load_font(font_path, 128)?;

  canvas.set_draw_color(Color::RGB(20, 20, 20));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump()?;
  let mut turn = Turn { team: Team::Red };
  let mut frame: u128 = 0;
  let mut change_turn: bool;

  loop {
    change_turn = false;

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => return Ok(()),
        Event::KeyDown {
          keycode: Some(Keycode::T),
          ..
        } => { change_turn = true },
        _ => {}
      }
    }

    canvas.set_draw_color(Color::RGB(20, 20, 20));
    canvas.clear();

    let is_red_turn: bool = turn.team == Team::Red;
    let is_blue_turn: bool = turn.team == Team::Blue;

    // Draw Tile
    Tile { x: 0 }.draw(&mut canvas);
    Tile { x: 1 }.draw(&mut canvas);
    Unit {
      x: 0,
      team: Team::Red,
    }
    .draw(&mut canvas, is_red_turn);
    Unit {
      x: 1,
      team: Team::Blue,
    }
    .draw(&mut canvas, is_blue_turn);

    canvas.set_draw_color(turn.team.light_color());
    let _ = canvas.fill_rect(Rect::new(0, 590, 800, 10));

    // Draw text
    let surface = font
      .render(&format!("Hello Rust! frame: {frame}, Current turn: {:?}", turn.team))
      .blended(Color::RGBA(255, 0, 0, 255))
      .map_err(|e| e.to_string())?;
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())?;

    let target = rect!(80, 80, 400, 50);
    canvas.copy(&texture, None, Some(target))?;
    // End draw text

    canvas.present();

    if change_turn {
      turn = turn.next();
    }

    frame += 1;
    sleep(Duration::from_millis(50));
  }
}

pub const TILE_SIZE: u32 = 64;
pub const UNIT_SIZE: u32 = 48;

#[derive(Debug)]
struct Turn {
  team: Team,
}

impl Turn {
  const fn next(&self) -> Self {
    match self.team {
      Team::Red => Self { team: Team::Blue },
      Team::Blue => Self { team: Team::Red },
    }
  }
}

