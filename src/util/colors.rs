pub fn red<S: AsRef<str>>(s: S, bg: bool) -> String {
    match bg {
        true => format!("\x1b[48;5;203;38;5;232;1m {} \x1b[0m", s.as_ref()),
        false => format!("\x1b[38;5;203;1m{}\x1b[0m", s.as_ref())
    }
}

pub fn blue<S: AsRef<str>>(s: S, bg: bool) -> String {
    match bg {
        true => format!("\x1b[48;5;31;38;5;232;1m {} \x1b[0m", s.as_ref()),
        false => format!("\x1b[38;5;31;1m{}\x1b[0m", s.as_ref())
    }
}

pub fn gray<S: AsRef<str>>(s: S, bg: bool) -> String {
    match bg {
        true => format!("\x1b[48;5;240;38;5;248;1m {} \x1b[0m", s.as_ref()),
        false => format!("\x1b[38;5;240;1m{}\x1b[0m", s.as_ref())
    }
}

pub fn bold<S: AsRef<str>>(s: S) -> String {
    format!("\x1b[1m {} \x1b[0m", s.as_ref())
}
