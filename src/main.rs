use crossterm::{
    execute as exec,
    queue,
    event::{ self, Event, KeyCode as Key, KeyModifiers },
    terminal as term,
};
use tokio::sync::mpsc;
use std::{
    io,
    fmt,
    time::{Duration, Instant},
    thread::sleep,
    collections::HashMap,
};

pub enum Input {
    Char(char),
}

#[derive(Clone, Copy)]
pub struct Rgb(u8, u8, u8);

impl Default for Rgb {
    fn default() -> Self {
        Rgb(80, 80, 255)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Rgb(r, g, b) = self;
        write!(f, "{r};{g};{b}")
    }
}

pub struct Pair(Option<Rgb>, Option<Rgb>);

pub const SQUARE: char = '\u{2580}';

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[38;2;{};48;2;{}m{}\x1b[49;39m", self.0.unwrap_or_default(), self.1.unwrap_or_default(), SQUARE)
    }
}

macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {
        ::std::collections::HashMap::from([ $(($k, $v)),* ])
    };
}

fn main() -> anyhow::Result<()> {
    let default_bg = Rgb(0, 0, 0);
    // let colors = HashMap::from([
    //     ('1', Rgb(255,  80,  80)),
    //     ('2', Rgb(255, 255,  80)),
    //     ('3', Rgb( 80, 255,  80)),
    // ]);

    let colors = hashmap! {
        b'1' => Rgb(020, 020, 020),
        b'2' => Rgb(240, 240, 240),
        b'3' => Rgb(180, 180, 180),
        b'4' => Rgb(255, 255, 080), // yellow
        b'5' => Rgb(080, 255, 080), // green
        b'6' => Rgb(255, 140, 160), // pink
    };

    let file = include_str!("../assets/test.txt");

    println!("{file}");

    let mut lines = file.lines();
    // while let Some(line1)
    // for i in 0..file.len() {}

    let mut pair_lines: Vec<Vec<Pair>> = Vec::new();
    let lines: Vec<&str> = file.lines().collect();
    for y in 0..lines.len() / 2 {
        let y = y * 2;
        let mut pairs: Vec<Pair> = Vec::new();
        for x in 0..lines[y].len() / 2 {
            let x = x * 2;
            pairs.push(Pair(
                colors.get(&lines[y].as_bytes()[x]).copied(),
                lines.get(y + 1).and_then(|l| l.as_bytes().get(x)).and_then(|l| colors.get(l)).copied(),
            ));
            pairs.push(Pair(
                colors.get(&lines[y].as_bytes()[x + 1]).copied(),
                lines.get(y + 1).and_then(|l| l.as_bytes().get(x + 1)).and_then(|l| colors.get(l)).copied(),
            ));
        }
        pair_lines.push(pairs);
    }

    for line in pair_lines.iter() {
        let mut display = String::new();
        for pair in line {
            display.push_str(pair.to_string().as_str());
        }
        println!("{display}");
    }

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

    return Ok(());

    let frame_time = Duration::from_nanos(1_000_000_000 / 60);

    let mut stdout = io::stdout();
    term::enable_raw_mode()?;
    exec!(stdout, term::EnterAlternateScreen)?;

    let mut dims: (u16, u16) = term::size()?;

    'main: loop {
        let start = Instant::now();

        let mut inputs: Vec<Input> = Vec::new();
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Resize(w, h) => {
                    dims = (w, h);
                }
                Event::Key(key) => {
                    match key.code {
                        Key::Char('c') => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                break 'main;
                            }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {}
                _ => {}
            }
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
