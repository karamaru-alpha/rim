use core::fmt::Display;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

pub struct Terminal {}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {
            height: height as usize,
            width: width as usize,
        })
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue(MoveTo(position.x as u16, position.y as u16))
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue(Hide)
    }

    pub fn print<T: Display>(v: T) -> Result<(), Error> {
        Self::queue(Print(v))
    }

    fn queue<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}
