extern crate pres;

use std::io::{stdin, stdout, Write};

use pres::event::{Event, Key, MouseEvent};
use pres::input::{MouseTerminal, TermRead};
use pres::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout,
           "{}{}q to exit. Click, click, click!",
           pres::clear::All,
           pres::cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", pres::cursor::Goto(x, y)).unwrap();
                    }
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
