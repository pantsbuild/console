use std::io;

use crate::term::Term;

pub fn move_cursor_down(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{}B", n))
    } else {
        Ok(())
    }
}

pub fn move_cursor_up(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{}A", n))
    } else {
        Ok(())
    }
}

#[inline]
pub fn move_cursor_to(out: &Term, x: usize, y: usize) -> io::Result<()> {
    out.write_str(&format!("\x1B[{};{}H", y + 1, x + 1))
}

pub fn clear_chars(out: &Term, n: usize) -> io::Result<()> {
    if n > 0 {
        out.write_str(&format!("\x1b[{}D\x1b[0K", n))
    } else {
        Ok(())
    }
}

#[inline]
pub fn clear_line(out: &Term) -> io::Result<()> {
    out.write_str("\r\x1b[2K")
}

#[inline]
pub fn clear_screen(out: &Term) -> io::Result<()> {
    out.write_str("\r\x1b[2J\r\x1b[H")
}

pub fn show_cursor(out: &Term) -> io::Result<()> {
    let esc = "\u{001B}";
    out.write_str(&format!("{}[?25h", esc))
}

pub fn hide_cursor(out: &Term) -> io::Result<()> {
    let esc = "\u{001B}";
    out.write_str(&format!("{}[?25l", esc))
}
