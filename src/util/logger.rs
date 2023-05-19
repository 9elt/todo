// @todo @low {fix single line message starting with \n}

use super::colors::{blue, bold, gray, red};

pub struct Logger {
    high: usize,
    normal: usize,
    low: usize,
    curr: usize,
}

const LABELS: [&str; 3] = ["IMPORTANT", "TODOs", "OTHER"];

impl Logger {
    pub fn new(high: usize, normal: usize, low: usize) -> Self {
        Self {
            high,
            normal,
            low,
            curr: 0,
        }
    }

    pub fn filname<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        println!("\n{} ", self.color(name, true));
        self
    }

    pub fn res(&mut self, res: Vec<(String, u32)>) -> &mut Self {
        if res.len() == 0 {
            self.curr += 1;
            return self
        }

        println!(
            "{}\n{}{} ",
            self.color("│", false),
            self.color("└──", false),
            self.color(LABELS[self.curr], true)
        );

        for (message, line) in res {
            println!(
                "{}  {} line {line}:\n{}{:04}{}",
                self.next_color("│", false),
                self.color("✪", false),
                self.next_color("│", false),
                " ",
                self.message(message)
            );
        }

        self.curr += 1;
        self
    }

    pub fn line(&mut self) -> &mut Self {
        println!();
        self
    }

    fn message(&mut self, message: String) -> String {
        let lines = message.lines().collect::<Vec<&str>>();

        let mut i = 0;
        let mut l = lines[i].trim();
        while i < lines.len() - 1 && l == "" {
            l = lines[i].trim();
            i += 1;
        }

        let mut s = bold(l);

        if i == 0 {
            i += 1;
        }

        for i in i..lines.len() {
            s = format!(
                "{s}\n{}{:04}{}",
                self.next_color("│", false),
                " ",
                bold(lines[i].trim())
            ); 
        }

        s
    }

    fn color<S: AsRef<str>>(&mut self, text: S, bg: bool) -> String {
        self.any_color(text, bg, self.curr)
    }

    fn next_color<S: AsRef<str>>(&mut self, text: S, bg: bool) -> String {
        self.any_color(text, bg, self.curr + 1)
    }

    fn any_color<S: AsRef<str>>(&mut self, text: S, bg: bool, curr: usize) -> String {
        if curr < 1 && self.high > 0 {
            format!("{}", red(text, bg))
        } else if curr < 2 && self.normal > 0 {
            format!("{}", blue(text, bg))
        } else if curr < 3 && self.low > 0 {
            format!("{}", gray(text, bg))
        } else {
            format!(" ")
        }
    }
}
