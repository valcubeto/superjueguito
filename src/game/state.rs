use crate::common::*;

pub struct GameState<'a> {
    screen: Screen<'a>,
}

impl GameState<'_> {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GameState {
            screen: Screen::new()?,
        })
    }

    #[inline(always)]
    pub fn set_dims(&mut self, w: u16, h: u16) -> anyhow::Result<()> {
        self.screen.set_dims(w, h)
    }

    #[inline(always)]
    pub fn ui_message(&mut self, pos: Position, msg: &str) -> anyhow::Result<()> {
        self.screen.draw_text(pos, msg)
    }
}
