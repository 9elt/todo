// @todo {allow any whitespace between todo and priority keys}

use std::slice::Iter;

pub fn parse(bytes: &[u8], from: &str, to: &str) -> Vec<(String, u32)> {
    Parser::new(bytes.iter(), from, to).parse().to_owned()
}

pub struct Parser<'a> {
    target: Iter<'a, u8>,
    result: Vec<(String, u32)>,
    from: &'a [u8],
    to: u8,
    line: u32,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: Iter<'a, u8>, from: &'a str, to: &'a str) -> Self {
        Parser {
            target: bytes,
            result: vec![],
            from: from.as_bytes(),
            to: to.as_bytes()[0],
            line: 1,
        }
    }

    pub fn parse(&mut self) -> &Vec<(String, u32)> {
        let mut find = self.from.iter();

        let mut f = find.next().unwrap_or(&0);
        let mut t = match self.target.next() {
            Some(t) => t,
            None => return &self.result,
        };

        self.check_line(t);

        while t != f {
            t = match self.target.next() {
                Some(t) => t,
                None => return &self.result,
            };

            self.check_line(t);
        }

        while t == f {
            f = find.next().unwrap_or(&0);
            t = match self.target.next() {
                Some(t) => t,
                None => return &self.result,
            };
        }

        if f != &0 {
            return self.parse();
        }

        let mut msg = vec![];

        while t != &self.to {
            msg.push(*t);

            t = match self.target.next() {
                Some(t) => t,
                None => return &self.result,
            };
        }

        let message = String::from_utf8(msg)
            .unwrap_or("failed parsing message".to_string());

        self.result.push((message, self.line));

        self.parse()
    }

    fn check_line(&mut self, t: &u8) {
        if t == &10 || t == &13 {
            self.line += 1 as u32
        }
    }
}
