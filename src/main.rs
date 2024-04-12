use {
  sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect}, ::std::thread::sleep, std::time::Duration
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
    canvas.set_draw_color(Color::RGB(40, 200, 40));
    let _ = canvas.fill_rect(Rect::new(0, 0, 64, 64));


    canvas.present();
    sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // The rest of the game loop goes here...
  }
}
