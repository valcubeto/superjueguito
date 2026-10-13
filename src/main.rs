mod common;
mod strings;
mod render;
mod game;

use common::*;
use ct::event::{ Event, KeyEvent, KeyCode, KeyModifiers, MouseEvent };
use tokio::sync::mpsc;
use std::thread::sleep;

use crate::game::state::GameState;

pub enum Input {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(Box<str>),
}

// pub struct Pair(Option<Rgb>, Option<Rgb>);

// impl fmt::Display for Pair {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\x1b[38;2;{};48;2;{}m{}\x1b[49;39m", self.0.unwrap_or_default(), self.1.unwrap_or_default(), PIXEL_CHAR)
//     }
// }

fn main() -> anyhow::Result<()> {
    // let colors = hashmap! {
    //     b'B' => Pixel::from(0x111111ff), // black
    //     b'D' => Pixel::from(0x404040ff), // dark gray
    //     b'G' => Pixel::from(0x808080ff), // gray
    //     b'L' => Pixel::from(0xbbbbbbff), // light gray
    //     b'W' => Pixel::from(0xf0f0f0ff), // white
    //     b'r' => Pixel::from(0xff5050ff), // red
    //     b'o' => Pixel::from(0xffaa50ff), // orange
    //     b'y' => Pixel::from(0xffff50ff), // yellow
    //     b'l' => Pixel::from(0xaaff50ff), // lime
    //     b'g' => Pixel::from(0x50ff50ff), // green
    //     b't' => Pixel::from(0x50ffaaff), // turquoise
    //     b'c' => Pixel::from(0x50ffffff), // cyan
    //     b'd' => Pixel::from(0x50aaffff), // dodger blue
    //     b'b' => Pixel::from(0x5050ffff), // blue
    //     b'v' => Pixel::from(0xaa50ffff), // violet
    //     b'm' => Pixel::from(0xff50ffff), // magenta
    //     b'p' => Pixel::from(0xff50aaff), // pink
    // };

    // let mut lines = file.lines();
    // while let Some(line1)
    // for i in 0..file.len() {}

    // let mut pair_lines: Vec<Vec<Pair>> = Vec::new();
    // let lines: Vec<&str> = file.lines().collect();
    // for y in 0..lines.len() / 2 {
    //     let y = y * 2;
    //     let mut pairs: Vec<Pair> = Vec::new();
    //     for x in 0..lines[y].len() / 2 {
    //         let x = x * 2;
    //         pairs.push(Pair(
    //             colors.get(&lines[y].as_bytes()[x]).copied(),
    //             lines.get(y + 1).and_then(|l| l.as_bytes().get(x)).and_then(|l| colors.get(l)).copied(),
    //         ));
    //         pairs.push(Pair(
    //             colors.get(&lines[y].as_bytes()[x + 1]).copied(),
    //             lines.get(y + 1).and_then(|l| l.as_bytes().get(x + 1)).and_then(|l| colors.get(l)).copied(),
    //         ));
    //     }
    //     pair_lines.push(pairs);
    // }

    // for line in pair_lines.iter() {
    //     let mut display = String::new();
    //     for pair in line {
    //         display.push_str(pair.to_string().as_str());
    //     }
    //     println!("{display}");
    // }

    // loop {
    //     let mut line1 = match lines.next() { Some(line) => line, None => break }.chars();
    //     let mut line2 = lines.next().unwrap_or_default().chars();
    //     loop {
    //         let ch1 = line1.
    //         let ch2 =
    //     }
    // }
    // while let Some(line) = lines.next() {
    //     let line2 = lines.next().unwrap_or_default();
    //     let mut chars = line.chars();
    //     while let Some(ch) = chars.next() {
    //         let ch2 = chars.next().unwrap_or_default();
    //         pairs.push(Pair());
    //         pairs.push()
    //     }
    // }

    let mut stdout = io::stdout();

    if !ct::tty::IsTty::is_tty(&stdout) {
        eprintln!("run this in a tty.");
        std::process::exit(1);
    }

    let sprite = Sprite::parse(include_str!("../assets/test.sprite"))?;
    println!("{sprite}");
    return Ok(());

    // ~ 60 fps by default.
    // ~ note that using thread::sleep() will cause very unstable fps.
    let frame_time = Duration::from_nanos(1_000_000_000 / 60);
    term::enable_raw_mode()?;
    exec!(stdout, term::EnterAlternateScreen)?;

    let mut game = GameState::new()?;

    'main: loop {
        let start = Instant::now();

        let mut inputs: Vec<Input> = Vec::new();
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Resize(w, h) => {
                    game.set_dims(w, h);
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

    exec!(stdout, term::LeaveAlternateScreen)?;
    term::disable_raw_mode()?;

    Ok(())
}
