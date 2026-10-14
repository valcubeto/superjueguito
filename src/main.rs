mod common;
mod strings;
mod render;
mod game;

use common::*;
use ct::event::{ Event, KeyEvent, KeyCode, KeyModifiers, MouseEvent };
use tokio::sync::mpsc;
use std::{ io::IsTerminal, thread::sleep, process::exit };

use crate::game::state::GameState;

pub enum Input {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(Box<str>),
}

fn main() -> anyhow::Result<()> {
    if !io::stdout().is_terminal() {
        eprintln!("run this in a tty.");
        exit(1);
    }

    let sprite = Sprite::parse(include_str!("../assets/kitten.sprite"))?;
    println!("{sprite}");
    return Ok(());

    // ~ 60 fps by default.
    // ~ note that using thread::sleep() will cause very unstable fps.
    let frame_time = Duration::from_nanos(1_000_000_000 / 60);

    let mut game = GameState::new()?;

    'main: loop {
        let start = Instant::now();

        let mut inputs: Vec<Input> = Vec::new();
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Resize(w, h) => {
                    game.set_dims(w, h)?;
                }
                Event::Key(key) => {
                    if let KeyCode::Char('c') = key.code && key.modifiers == KeyModifiers::CONTROL {
                        break 'main;
                    }
                    inputs.push(Input::Key(key));
                }
                Event::Mouse(mouse) => {
                    inputs.push(Input::Mouse(mouse))
                }
                Event::Paste(text) => {
                    inputs.push(Input::Paste(text.into_boxed_str()))
                }
                _ => {}
            }
        }

        for input in inputs {
            // match input {
            //     Input::Key(key) => match key.code {
            //         // KeyCode::Esc => game.toggle_menu(),
            //         KeyCode::Left => game.ui_message(Position::from((2, 1)), "left"),
            //         _ => {}
            //     }
            //     _ => {}
            // }
        }

        // update game state
        // drain network messages
        // render

        if let Some(remaining) = frame_time.checked_sub(start.elapsed()) {
            sleep(remaining);
        }
    }

    Ok(())
}
