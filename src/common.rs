//! Commonly imported dependencies.

#![allow(unused_imports)]

pub use crate::{
    render::{
        screen::{ Screen, Vec2d, Position },
        sprite::{ Pixel, Sprite },
    },
    strings::*,
};

pub use crossterm::{
    self as ct,
    execute as exec,
    queue,
    terminal as term,
    event,
};

pub use std::{
    io::{ self, Write },
    fmt,
    time::{ Duration, Instant },
    collections::HashMap,
};

pub use anyhow::Context;

pub use itertools::*;

#[doc(hidden)]
pub mod _macros {
    #[macro_export]
    macro_rules! hashmap {
        ($($k:expr => $v:expr),* $(,)?) => {
            ::std::collections::HashMap::from([ $(($k, $v)),* ])
        };
    }
}

pub use _macros::*;
