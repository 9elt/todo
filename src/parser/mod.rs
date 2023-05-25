use std::slice::Iter;
mod consts;
pub mod priority;

use consts::*;
use core::result::Result as CoreResult;
use priority::Priority;

type Result<T> = CoreResult<T, ParserError>;

pub struct Parser<'a> {
    original: &'a str,

    target: Iter<'a, u8>,
    cursor: &'a u8,

    result: Vec<(String, u32, Priority)>,
    line: u32,
}

#[derive(Debug)]
enum ParserError {
    Completed,
    Incorrect,
    NotAllowed,
    Missing,
    Collection,
}

impl ParserError {
    fn is_completed(&self) -> bool {
        match self {
            Self::Completed => true,
            _ => false,
        }
    }
}

use Priority::*;

fn ranges(msg: &String) -> Result<Vec<u32>> {
    let mut parser = Parser::new(msg.as_str());

    parser.move_to("[", ANY)?;
    let range = parser.collect_to("]", ANY)?;

    Ok(range
        .split(":")
        .into_iter()
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .collect())
}

const PRIORITY_KEYS: &[&str] = &["@low", "@moderate", "@high"];
const PRIORITIES: &[Priority] = &[LOW, MODERATE, HIGH];

pub fn test() {
    let mut parser = Parser::new("@todo   @high {wowo [32:32]}  ");

    match parser.move_to("@todo", ANY) {
        Ok(_) => (),
        Err(_) => (/* return result */),
    };

    let priority = match parser.find_to(PRIORITY_KEYS, "{") {
        Ok(i) => &PRIORITIES[i],
        Err(ParserError::Missing) => &PRIORITIES[1],
        Err(_) => return, /* continue parsing */
    };

    let message = match parser.collect_to("}", ANY) {
        Ok(message) => message,
        Err(_) => return,
    };

    let ranges = ranges(&message).unwrap();

    println!("-> priority: {:?}, message: {:?} - {:?}", priority, message, ranges);
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a str) -> Self {
        let mut new = Self {
            original: file,
            target: file.as_bytes().iter(),
            cursor: &0,
            result: vec![],
            line: 1,
        };

        new.incr_cursor().unwrap();

        new
    }

    fn find_to(&mut self, options: &[&str], to: &str) -> Result<usize> {
        let limit = &to.as_bytes()[0];

        let mut targets = options.iter().map(|b| b.as_bytes().iter()).collect::<Vec<_>>();
        let mut cursors = targets.iter_mut().map(|i| i.next()).collect::<Vec<_>>();
        let mut res = cursors.iter().map(|_| None).collect::<Vec<_>>();

        self.incr_cursor()?;

        while self.cursor != limit {
            let mut is_matching = false;

            for i in 0..res.len() {
                if cursors[i].is_none() {
                    continue;
                }

                if res[i].is_some() {
                    res[i] = Some(Some(self.cursor) == cursors[i]);
                } else if Some(self.cursor) == cursors[i] {
                    res[i] = Some(true)
                }

                if res[i].is_some() {
                    cursors[i] = targets[i].next();
                    if res[i].unwrap() {
                        is_matching = true;
                    }
                }
            }

            if !is_matching {
                self.is_whitespace()?;
            }

            self.incr_cursor()?;
        }

        self.incr_cursor()?;

        if !res.iter().any(|v| v.is_some()) {
            return Err(ParserError::Missing);
        }

        match res.iter().position(|r| r.is_some() && r.unwrap()) {
            Some(index) => Ok(index),
            None => Err(ParserError::Incorrect),
        }
    }

    fn collect_to(&mut self, to: &str, allow: &[u8]) -> Result<String> {
        let mut res = vec![];

        let mut target = to.as_bytes().iter();
        let target_cursor = target.next();

        while Some(self.cursor) != target_cursor {
            if allow.len() > 0 && !allow.contains(self.cursor) {
                return Err(ParserError::NotAllowed);
            }

            res.push(self.cursor.to_owned());

            self.incr_cursor()?;
        }

        match String::from_utf8(res) {
            Ok(res) => Ok(res),
            Err(_) => Err(ParserError::Collection)
        }
    }

    fn move_by(&mut self, n: usize) -> Result<()> {
        for _ in 0..n {
            self.incr_cursor()?;
        }

        Ok(())
    }

    fn move_to(&mut self, to: &str, allow: &[u8]) -> Result<()> {
        let mut target = to.as_bytes().iter();
        let mut target_cursor = target.next();

        while Some(self.cursor) != target_cursor {
            if allow.len() > 0 && !allow.contains(self.cursor) {
                return Err(ParserError::NotAllowed);
            }

            self.incr_cursor()?;
        }

        while Some(self.cursor) == target_cursor {
            target_cursor = target.next();
            self.incr_cursor()?
        }

        if target_cursor.is_some() {
            self.move_to(to, allow)
        } else {
            Ok(())
        }
    }

    fn is_whitespace(&mut self) -> Result<()> {
        if !WHITE_SPACE.contains(self.cursor) {
            Err(ParserError::Incorrect)
        } else {
            Ok(())
        }
    }

    fn incr_cursor(&mut self) -> Result<()> {
        self.cursor = match self.target.next() {
            Some(cursor) => cursor,
            None => return Err(ParserError::Completed),
        };

        self.update_line();

        Ok(())
    }

    fn update_line(&mut self) {
        if self.cursor == &10 || self.cursor == &13 {
            self.line += 1 as u32
        }
    }

    // fn result(&self) -> ParserResult {
    //     self.result.to_owned()
    // }

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

                let mut lp = "LOW".as_bytes().iter();
                let l = lp.next().unwrap_or(&0);

                let mut hp = "HIGH".as_bytes().iter();
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

    fn parse_ln_num(&self, v: &str) -> CoreResult<u32, std::num::ParseIntError> {
        if v.contains("+") {
            match u32::from_str_radix(&v.replace("+", ""), 10) {
                Ok(ln) => Ok(self.line + ln),
                Err(err) => Err(err),
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
