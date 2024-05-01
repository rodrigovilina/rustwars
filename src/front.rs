use sdl2::{
  pixels::Color,
  render::{Canvas, TextureCreator},
  ttf::Sdl2TtfContext,
  video::{Window, WindowContext},
  EventPump, Sdl, VideoSubsystem,
};

pub struct Frontend {
  pub canvas: Canvas<Window>,
  pub event_pump: EventPump,
  pub frame: u128,
  pub texture_creator: TextureCreator<WindowContext>,
  pub ttf_context: Sdl2TtfContext,
}

impl Frontend {
  pub fn new() -> Result<Self, String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let ttf_context: Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window: Window = video_subsystem
      .window("rustwars", 800, 600)
      .position_centered()
      .opengl()
      .build()
      .map_err(|e| e.to_string())?;
    let canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let event_pump: EventPump = sdl_context.event_pump()?;

    Ok(Self {
      canvas,
      event_pump,
      frame: 0,
      texture_creator,
      ttf_context,
    })
  }

  pub fn clear_and_present_black(&mut self) {
    self.canvas.set_draw_color(Color::RGB(20, 20, 20));
    self.canvas.clear();
    self.canvas.present();
  }
}
