use crate::parser::priority::Priority;

pub struct ResultLogger {
    result: Vec<(String, u32, Priority)>,
}

impl ResultLogger {
    pub fn new(result: Vec<(String, u32, Priority)>) -> Self {
        Self { result }
    }

    pub fn filname<S: AsRef<str>>(&self, name: S) -> &Self {
        println!("\n{} ", self.result[0].2.background(name));
        self
    }

    pub fn res(&self) -> &Self {
        for i in 0..self.result.len() {
            let (message, line, priority) = &self.result[i];

            let next_priority = match i + 1 < self.result.len() {
                true => Some(&self.result[i + 1].2),
                false => None,
            };

            println!(
                "{}\n{}{}{}",
                priority.color("│"),
                priority.color("└─ᐅ"),
                priority.background(line.to_string()),
                self.message(message.to_owned(), next_priority)
            );
        }

        self
    }

    pub fn line(&self) -> &Self {
        println!();
        self
    }

    fn message(&self, message: String, next_priority: Option<&Priority>) -> String {
        let lines = message
            .lines()
            .map(|v| v.trim())
            .filter(|v| v != &"")
            .map(|v| self.replace_refs(v))
            .collect::<Vec<String>>();

        let mut s = self.bold(&lines[0]);

        for i in 1..lines.len() {
            s = format!(
                "{s}\n{} {}",
                match next_priority {
                    Some(p) => p.color("│"),
                    None => " ".to_string(),
                },
                self.bold(&lines[i])
            );
        }

        s
    }

    fn bold<S: AsRef<str>>(&self, s: S) -> String {
        format!("\x1b[1m {}\x1b[0m", s.as_ref())
    }

    fn replace_refs(&self, v: &str) -> String {
        v.replace("ref*", "\x1b[0m")
            .replace("*ref", "\x1b[0m")
            .replace("lcnt*", "\x1b[0m\x1b[38;5;240m")
            .replace("*lcnt", "|\x1b[0m")
    }
}
