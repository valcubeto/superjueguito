use crate::common::*;

#[derive(Clone, Copy)]
pub struct Dimentions { w: u16, h: u16 }

impl From<(u16, u16)> for Dimentions {
    fn from((w, h): (u16, u16)) -> Self {
        Dimentions { w, h }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Vec2d { x: i16, y: i16 }

#[derive(Clone, Copy)]
pub enum AlignX {
    /// negative values go outside the screen, left or top.
    Left(i16),
    /// negative values go outside the screen, right or bottom.
    Right(i16),
    /// auto-align to left or top.
    Before,
    /// auto-align to right or bottom.
    After,
    /// auto-align to center.
    Center,
}

#[derive(Clone, Copy)]
pub enum AlignY {
    /// negative values go outside the screen, left or top.
    Top(i16),
    /// negative values go outside the screen, right or bottom.
    Bottom(i16),
    /// auto-align to left or top.
    Before,
    /// auto-align to right or bottom.
    After,
    /// auto-align to center.
    Center,
}

#[derive(Clone, Copy)]
pub struct Position { x: AlignX, y: AlignY }

impl From<(i16, i16)> for Position {
    fn from((x, y): (i16, i16)) -> Self {
        Position {
            x: AlignX::Left(x),
            y: AlignY::Top(y),
        }
    }
}

pub struct Layer<'a> {
    pub elements: Vec<(Position, Sprite<'a>)>,
}

impl Layer<'_> {
    // pub fn render(&self) -> anyhow::Result<()> {
    //     for element in self.elements.iter() {
    //         //
    //     }
    //     Ok(())
    // }
}

impl Default for Layer<'_> {
    fn default() -> Self {
        Layer {
            elements: vec![]
        }
    }
}

pub struct Screen<'a> {
    /// virtual cursor to render outside the screen.
    // vcursor: Vec2d,
    dimentions: Dimentions,
    layers: Vec<Layer<'a>>,
    buffer: Vec<Vec<Option<Box<str>>>>
}

impl Screen<'_> {
    pub fn new() -> anyhow::Result<Self> {
        term::enable_raw_mode()?;
        exec!(io::stdout(), term::EnterAlternateScreen)?;

        Ok(Screen {
            dimentions: term::size()?.into(),
            layers: vec![Layer::default()],
            buffer: vec![]
        })
    }

    fn calculate_pos(&self, pos: &Position, elem_dims: Dimentions) -> (u16, u16) {
        let x = match pos.x {
            AlignX::Left(n) if n < 0 => {
                todo!()
            }
            AlignX::Left(n) => {
                0
            }
            AlignX::Right(n) if n < 0 => {
                todo!()
            }
            AlignX::Right(n) => {
                todo!()
            }
            AlignX::After => {
                todo!()
            }
            AlignX::Before => {
                todo!()
            }
            AlignX::Center => {
                todo!()
            }
        };
        let y = match pos.y {
            AlignY::Top(n) if n < 0 => {
                todo!()
            }
            AlignY::Top(n) => {
                0
            }
            AlignY::Bottom(n) if n < 0 => {
                todo!()
            }
            AlignY::Bottom(n) => {
                todo!()
            }
            AlignY::After => {
                todo!()
            }
            AlignY::Before => {
                todo!()
            }
            AlignY::Center => {
                todo!()
            }
        };
        (x, y)
    }

    pub fn render(&self) -> anyhow::Result<()> {
        for layer in self.layers.iter() {
            // layer.render()?;
            for (pos, sprite) in layer.elements.iter() {
                let mut buf_lines = sprite.buffer.iter();
                while let Some(buf_line_hi) = buf_lines.next() {
                    if let Some(buf_line_lo) = buf_lines.next() {} else {}
                }
                // self.buffer
                todo!();
            }
        }
        Ok(())
    }

    pub fn set_dims(&mut self, w: u16, h: u16) -> anyhow::Result<()> {
        self.dimentions = Dimentions  { w, h };
        self.render()?;
        Ok(())
    }

    pub fn draw_text(&mut self, pos: Position, text: &str) -> anyhow::Result<()> {
        let x: u16 = match pos.x {
            Align::Absolute(n) if n < 0 => 0,
            Align::Absolute(n) => n as u16,
            Align::After => 0,
            Align::Before => self.dimentions.w - (text.len().max(0xffff) as u16),
            Align::Center => self.dimentions.w / 2 - (text.len() as u16).max(self.dimentions.w) / 2,
            _ => todo!()
        };
        // self.layers.last()
        todo!();
        Ok(())
    }
}

impl Drop for Screen<'_> {
    fn drop(&mut self) {
        let _ = exec!(io::stdout(), term::LeaveAlternateScreen);
        let _ = term::disable_raw_mode();
    }
}
