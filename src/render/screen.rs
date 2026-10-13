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
pub enum Align {
    Absolute(i16),
    /// left or top.
    Before,
    /// right or bottom.
    After,
    Center,
}

#[derive(Clone, Copy)]
pub struct Position { x: Align, y: Align }

impl From<(i16, i16)> for Position {
    fn from((x, y): (i16, i16)) -> Self {
        Position {
            x: Align::Absolute(x),
            y: Align::Absolute(y),
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
    buffer: Vec<Vec<Box<str>>>
}

impl Screen<'_> {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Screen {
            dimentions: term::size()?.into(),
            layers: vec![Layer::default()],
            buffer: vec![]
        })
    }

    pub fn render(&self) -> anyhow::Result<()> {
        for layer in self.layers.iter() {
            // layer.render()?;
            for element in layer.elements.iter() {
                // for i in 0..element.0.y {}
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
            Align::Center => self.dimentions.w / 2 - (text.len() as u16).max(self.dimentions.w) / 2
        };
        // self.layers.last()
        todo!();
        Ok(())
    }
}
