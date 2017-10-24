//! Character stream used for parsing.

use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::mem;
use std::rc::Rc;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct CharStream {
    contents: Rc<String>,
    stream: Rc<RefCell<Peekable<Chars<'static>>>>,
    line: Rc<RefCell<usize>>,
    col: Rc<RefCell<usize>>,
}

impl CharStream {
    pub fn from_file(path: &str) -> io::Result<CharStream> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let chars: Chars = unsafe { mem::transmute(contents.chars()) };
        let stream = Rc::new(RefCell::new(chars.peekable()));
        let line = Rc::new(RefCell::new(1));
        let col = Rc::new(RefCell::new(1));
        let contents = Rc::new(contents);

        Ok(CharStream {
            contents,
            stream,
            line,
            col,
        })
    }

    pub fn peek(&self) -> Option<char> {
        let mut peekable = self.stream.borrow_mut();
        let opt = peekable.peek();

        match opt {
            Some(ref ch) => {
                // if cfg!(debug_assertions) {
                //     use super::misc::format_char;
                //     println!(
                //         "peeking. ch: '{}', line: {}, col: {}",
                //         &format!("{}", **ch),
                //         self.line(),
                //         self.col()
                //     );
                // }
                Some(**ch)
            }
            None => None,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let opt = {
            let mut peekable = self.stream.borrow_mut();
            peekable.next()
        };

        match opt {
            Some(ch) => {
                // if cfg!(debug_assertions) {
                //     use super::misc::format_char;
                //     println!(
                //         "ch: '{}', line: {}, col: {}",
                //         &format_char(&ch),
                //         self.line(),
                //         self.col()
                //     );
                // }
                if ch == '\n' {
                    let line = self.line();
                    self.set_line(line + 1);
                    self.set_col(1);
                } else {
                    let col = self.col();
                    self.set_col(col + 1);
                }
                Some(ch)
            }
            None => None,
        }
    }

    pub fn line(&self) -> usize {
        let line = self.line.borrow();
        *line
    }

    pub fn col(&self) -> usize {
        let col = self.col.borrow();
        *col
    }

    fn set_line(&mut self, value: usize) {
        let mut line = self.line.borrow_mut();
        *line = value;
    }

    fn set_col(&mut self, value: usize) {
        let mut col = self.col.borrow_mut();
        *col = value;
    }
}
