use std::io::{stdin, stdout, Error, Stdout, Write};
use termion::color::{Bg, Fg, Reset, Rgb};

use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y))
    }
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide)
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show)
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine)
    }
    pub fn set_bg_color(color: Rgb) {
        print!("{}", Bg(color));
    }
    pub fn reset_bg_color() {
        print!("{}", Bg(Reset))
    }
    pub fn set_fg_color(color: Rgb) {
        print!("{}", Fg(color))
    }
    pub fn reset_fg_color() {
        print!("{}", Fg(Reset))
    }
}
