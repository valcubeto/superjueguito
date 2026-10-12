use crossterm::{queue, execute as exec};
use crossterm::event::{self, Event, KeyCode as Key, KeyModifiers};
use crossterm::terminal::{self as term};
use tokio::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io;
use std::fmt;
use std::collections::HashMap;

pub enum Input {
	Char(char),
}

#[derive(Clone, Copy)]
pub struct Rgb(u8, u8, u8);

impl fmt::Display for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Rgb(r, g, b) = self;
		write!(f, "{r};{g};{b}")
	}
}

pub struct Pair(Option<Rgb>, Option<Rgb>);

macro_rules! hashmap {
	($($k:expr => $v:expr),* $(,)?) => {
		::std::collections::HashMap::from([ $(($k, $v)),* ])
	};
}

fn main() -> anyhow::Result<()> {
	let default_bg = Rgb(0, 0, 0);
	// let colors = HashMap::from([
	// 	('1', Rgb(255,  80,  80)),
	// 	('2', Rgb(255, 255,  80)),
	// 	('3', Rgb( 80, 255,  80)),
	// ]);

	let colors = hashmap! {
		'1' => Rgb( 20,  20,  20),
		'2' => Rgb(240, 240, 240),
		'3' => Rgb(180, 180, 180),
		'4' => Rgb(255, 255,  80), // yellow
		'5' => Rgb( 80, 255,  80), // green
		'6' => Rgb(255, 140, 160), // pink
	};

	let file = include_str!("../assets/test.txt");

	println!("{file}");

	let mut pairs: Vec<Option<Rgb>> = Vec::new();
	let mut lines = file.lines();
	loop {
		let mut line1 = match lines.next() { Some(line) => line, None => break }.chars();
		let mut line2 = lines.next().unwrap_or_default().chars();
		loop {
			let ch1 = line1.
			let ch2 = 
		}
	}
	while let Some(line) = lines.next() {
		let line2 = lines.next().unwrap_or_default();
		let mut chars = line.chars();
		while let Some(ch) = chars.next() {
			let ch2 = chars.next().unwrap_or_default();
			pairs.push(Pair());
			pairs.push()
		}
	}

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
