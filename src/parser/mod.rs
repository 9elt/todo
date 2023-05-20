use std::slice::Iter;
mod consts;
pub mod priority;

use consts::*;
use priority::Priority;

pub struct Parser<'a> {
    original: &'a str,
    target: Iter<'a, u8>,
    result: Vec<(String, u32, Priority)>,
    line: u32,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a str) -> Self {
        Parser {
            original: file,
            target: file.as_bytes().iter(),
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

        self.incr_line(t);

        let mut msg = vec![];

        let mut pushref = false;
        let mut refs = vec![];

        while t != me {
            if !COMMENTS.contains(t) {
                msg.push(*t);
            }

            if t == REF_START {
                pushref = true;
                refs.push(vec![]);
            } else if t == REF_END {
                pushref = false;
            }

            if pushref && t != REF_START {
                refs.last_mut().unwrap().push(*t);
            }

            t = match self.target.next() {
                Some(t) => t,
                None => return self.result.to_owned(),
            };

            self.incr_line(t);
        }

        let mut message = String::from_utf8(msg).unwrap_or("failed parsing message".to_string());

        if refs.len() > 0 {
            self.replace_refs(&mut message, refs);
        }

        self.result.push((message, self.line, priority));

        self.parse()
    }

    fn parse_ln_num(&self, v: &str) -> Result<u32, std::num::ParseIntError> {
        if v.contains("+") {
            match u32::from_str_radix(&v.replace("+", ""), 10) {
                Ok(ln) => Ok(self.line + ln),
                Err(err) => Err(err)
            }
        } else {
            u32::from_str_radix(v, 10)
        }
    }

    fn replace_refs(&mut self, message: &mut String, refs: Vec<Vec<u8>>) {
        for reference in refs {
            let ref_str = String::from_utf8(reference).unwrap();

            let r = ref_str
                .split(":")
                .map(|v| self.parse_ln_num(v))
                .filter_map(|v| v.ok())
                .collect::<Vec<u32>>();

            if r.len() > 0 {
                let mut refrange = vec![];

                if r.len() > 1 {
                    for i in r[0]..=r[1] {
                        refrange.push(i)
                    }
                } else {
                    refrange.push(r[0])
                }

                let mut refres = "\nref*".to_string();
                let mut cl = 1;

                for line in self.original.lines() {
                    if refrange.contains(&cl) {
                        refres = format!("{refres}\nlcnt*{:>3}*lcnt  ref*{line}*ref", cl);
                    }
                    cl += 1;
                }

                refres = format!("{refres}\nref*\n");
                
                *message = message.replace(&format!("[{ref_str}]"), &refres);
            }
        }
    }

    fn incr_line(&mut self, t: &u8) {
        if t == &10 || t == &13 {
            self.line += 1 as u32
        }
    }
}
