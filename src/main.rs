#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

use {
  sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window}, ::std::thread::sleep, std::time::Duration
};

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("rust-sdl2 demo: Video", 800, 600)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(Color::RGB(20, 20, 20));
  canvas.clear();
  canvas.present();
  let mut event_pump = sdl_context.event_pump()?;

  loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => return Ok(()),
        _ => {}
      }
    }

    canvas.set_draw_color(Color::RGB(20, 20, 20));
    canvas.clear();

    // Draw Tile
    canvas.set_draw_color(Color::RGB(60, 160, 40));
    Tile { x: 0 }.draw(&mut canvas);
    Tile { x: 1 }.draw(&mut canvas);

    canvas.present();
    sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // The rest of the game loop goes here...
  }
}

const TILE_SIZE: u32 = 64;

struct Tile {
  x: u16,
}

impl Tile {
  fn draw(&self, canvas: &mut Canvas<Window>) {
    let _ = canvas.fill_rect(Rect::new(i32::from((64 + 1) * self.x), 0, TILE_SIZE, TILE_SIZE));
  }
}
