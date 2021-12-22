use std::io;
use std::io::Read;

use crate::event;
use crate::event::{Key, MouseEvent};

/// An event reported by the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// A key press.
    Key(Key),
    /// A mouse button press, release or wheel use at specific coordinates.
    Mouse(MouseEvent),
    /// An event that cannot currently be evaluated.
    Unsupported(Vec<u8>),
}


/// An iterator over input events.
pub struct Events<R> {
    pub inner: EventsAndRaw<R>,
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<Event, io::Error>;

    fn next(&mut self) -> Option<Result<Event, io::Error>> {
        self.inner.next().map(|tuple| tuple.map(|(event, _raw)| event))
    }
}

/// An iterator over input events and the bytes that define them.
pub struct EventsAndRaw<R> {
    pub source: R,
    pub leftover: Option<u8>,
}

impl<R: Read> Iterator for EventsAndRaw<R> {
    type Item = Result<(Event, Vec<u8>), io::Error>;

    fn next(&mut self) -> Option<Result<(Event, Vec<u8>), io::Error>> {
        let source = &mut self.source;

        if let Some(c) = self.leftover {
            // we have a leftover byte, use it
            self.leftover = None;
            return Some(parse_event(c, &mut source.bytes()));
        }

        // Here we read two bytes at a time. We need to distinguish between single ESC key presses,
        // and escape sequences (which start with ESC or a x1B byte). The idea is that if this is
        // an escape sequence, we will read multiple bytes (the first byte being ESC) but if this
        // is a single ESC keypress, we will only read a single byte.
        let mut buf = [0u8; 2];
        let res = match source.read(&mut buf) {
            Ok(0) => return None,
            Ok(1) => {
                match buf[0] {
                    b'\x1B' => Ok((Event::Key(Key::Esc), vec![b'\x1B'])),
                    c => parse_event(c, &mut source.bytes()),
                }
            }
            Ok(2) => {
                let option_iter = &mut Some(buf[1]).into_iter();
                let result = {
                    let mut iter = option_iter.map(|c| Ok(c)).chain(source.bytes());
                    parse_event(buf[0], &mut iter)
                };
                // If the option_iter wasn't consumed, keep the byte for later.
                self.leftover = option_iter.next();
                result
            }
            Ok(_) => unreachable!(),
            Err(e) => Err(e),
        };

        Some(res)
    }
}

fn parse_event<I>(item: u8, iter: &mut I) -> Result<(Event, Vec<u8>), io::Error>
    where I: Iterator<Item=Result<u8, io::Error>>
{
    let mut buf = vec![item];
    let result = {
        let mut iter = iter.inspect(|byte| if let &Ok(byte) = byte {
            buf.push(byte);
        });
        event::parse_event(item, &mut iter)
    };
    result.or(Ok(Event::Unsupported(buf.clone()))).map(|e| (e, buf))
}
