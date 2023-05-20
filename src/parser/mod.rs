use std::slice::Iter;
pub mod priority;
mod consts;

use consts::*;
use priority::Priority;

pub struct Parser<'a> {
    target: Iter<'a, u8>,
    result: Vec<(String, u32, Priority)>,
    line: u32,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: Iter<'a, u8>) -> Self {
        Parser {
            target: bytes,
            result: vec![],
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Vec<(String, u32, Priority)> {
        let mut key = TODO_KEY.iter();
        let mut k = key.next().unwrap_or(&0);

        let mut t = match self.target.next() {
            Some(t) => t,
            None => return self.result.to_owned(),
        };

        self.incr_line(t);

        while t != k {
            t = match self.target.next() {
                Some(t) => t,
                None => return self.result.to_owned(),
            };

            self.incr_line(t);
        }

        while t == k {
            k = key.next().unwrap_or(&0);
            t = match self.target.next() {
                Some(t) => t,
                None => return self.result.to_owned(),
            };
        }

        if k != &0 {
            return self.parse();
        }

        let ms = MSG_START;
        let me = MSG_END;

        let mut priority = Priority::MODERATE;

        while t != ms {
            // find priority
            if t == AT_KEY {
                t = match self.target.next() {
                    Some(t) => t,
                    None => return self.result.to_owned(),
                };

                let mut lp = LOW.iter();
                let l = lp.next().unwrap_or(&0);

                let mut hp = HIGH.iter();
                let h = hp.next().unwrap_or(&0);

                let mut spr;
                let mut s;

                if t == l {
                    spr = lp;
                    s = l;
                    priority = Priority::LOW;
                } else if t == h {
                    spr = hp;
                    s = h;
                    priority = Priority::HIGH;
                } else {
                    return self.parse();
                }

                while t == s {
                    t = match self.target.next() {
                        Some(t) => t,
                        None => return self.result.to_owned(),
                    };

                    s = spr.next().unwrap_or(&0);
                }

                if s != &0 {
                    return self.parse();
                }
            }

            if !WHITE_SPACE.contains(t) {
                return self.parse();
            }

            t = match self.target.next() {
                Some(t) => t,
                None => return self.result.to_owned(),
            };

            self.incr_line(t);
        }

        t = match self.target.next() {
            Some(t) => t,
            None => return self.result.to_owned(),
        };

        let mut msg = vec![];

        while t != me {
            if !COMMENTS.contains(t) {
                msg.push(*t);
            }

            t = match self.target.next() {
                Some(t) => t,
                None => return self.result.to_owned(),
            };
        }

        let message = String::from_utf8(msg)
            .unwrap_or("failed parsing message".to_string());

        self.result.push((message, self.line, priority));

        self.parse()
    }

    fn incr_line(&mut self, t: &u8) {
        if t == &10 || t == &13 {
            self.line += 1 as u32
        }
    }
}
