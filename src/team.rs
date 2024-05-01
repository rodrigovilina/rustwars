use sdl2::{pixels::Color, render::Canvas, video::Window};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
  Red,
  Blue,
}

impl Team {
  pub fn set_canvas_color(self, canvas: &mut Canvas<Window>, light: bool) {
    let color = if light {
      self.light_color()
    } else {
      self.dark_color()
    };

    canvas.set_draw_color(color);
  }

  pub const fn dark_color(self) -> Color {
    match self {
      Self::Red => Color::RGB(100, 10, 10),
      Self::Blue => Color::RGB(10, 10, 100),
    }
  }

  pub const fn light_color(self) -> Color {
    match self {
      Self::Red => Color::RGB(180, 60, 60),
      Self::Blue => Color::RGB(60, 60, 180),
    }
  }
}
