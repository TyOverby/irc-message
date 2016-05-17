use std::collections::hash_map::HashMap;
use std::hash::Hash;
use std::convert::AsRef;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

#[cfg(test)]
mod tests;

macro_rules! try_o {
    ($a: expr) => {
        match $a {
            Some(x) => x,
            None => return None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IrcMessage<T: Eq + Hash + AsRef<str>> {
    pub raw: T,
    pub tags: HashMap<T, Option<T>>,
    pub prefix: Option<T>,
    pub command: Option<T>,
    pub params: Vec<T>
}

impl <T: Eq + Hash + AsRef<str>> Display for IrcMessage<T> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        if !self.tags.is_empty() {
            try!(fmt.write_str("@"));
            for (k, v) in self.tags.iter() {
                match v {
                    &None => {
                        try!(fmt.write_str(k.as_ref()));
                        try!(fmt.write_str(";"));
                    },
                    &Some(ref val) => {
                        try!(fmt.write_str(k.as_ref()));
                        try!(fmt.write_str("="));
                        try!(fmt.write_str(val.as_ref()));
                        try!(fmt.write_str(";"));
                    }
                }
            }

            if self.prefix.is_some() || self.command.is_some() || !self.params.is_empty() {
                try!(fmt.write_str(" "));
            }
        }

        if let Some(prefix) = self.prefix.as_ref() {
            try!(fmt.write_str(":"));
            try!(fmt.write_str(prefix.as_ref()));

            if self.command.is_some() || !self.params.is_empty() {
                try!(fmt.write_str(" "));
            }
        }

        if let Some(command) = self.command.as_ref() {
            try!(fmt.write_str(command.as_ref()));
            if !self.params.is_empty() {
                try!(fmt.write_str(" "));
            }
        }

        let length = self.params.len();
        for (i, param) in self.params.iter().enumerate() {
            if param.as_ref().contains(' ') ||
               (param.as_ref().chars().nth(0) == Some(':') && i == length - 1) {
                   try!(fmt.write_str(":"));
                   try!(fmt.write_str(param.as_ref()));
                   break;
            } else {
                try!(fmt.write_str(param.as_ref()));
            }

            if i != length - 1 {
                try!(fmt.write_str(" "));
            }
        }
        Ok(())
    }
}

impl IrcMessage<String> {
    pub fn parse_own(line: &str) -> Option<IrcMessage<String>> {
        parse_into(line, |a| a.to_string())
    }
}

impl <'a> IrcMessage<&'a str> {
    pub fn parse_ref(line: &'a str) -> Option<IrcMessage<&'a str>> {
        parse_into(line, |a| a)
    }
}

fn next_segment(line: &str) -> Option<(&str, &str)> {
    match line.find(' ') {
        Some(n) => {
            let segment = &line[..n];
            let rest = &line[n..];
            let rest = rest.trim_left();
            Some((segment, rest))
        },
        None => None
    }
}

fn parse_into<'a, T: Eq + Hash + AsRef<str>, F>(line: &'a str, wrap: F) -> Option<IrcMessage<T>>
where F: Fn(&'a str) -> T {
    let mut message = IrcMessage {
        raw: wrap(line),
        tags: HashMap::new(),
        prefix: None,
        command: None,
        params: vec![]
    };

    // TAGS
    let line = if try_o!(line.chars().next()) == '@' {
        let (tags, rest) = try_o!(next_segment(&line[1..]));
        let raw_tags = tags.split(';');
        for tag in raw_tags {
            if tag.contains('=') {
                let mut pair = tag.split('=');
                message.tags.insert(wrap(try_o!(pair.next())),
                                    Some(wrap(try_o!(pair.next()))));
            } else {
                message.tags.insert(wrap(tag), None);
            }
        }
        rest
    } else {
        line
    };

    // PREFIX
    let line = if try_o!(line.chars().next()) == ':' {
        let (prefix, rest) = try_o!(next_segment(&line[1..]));
        message.prefix = Some(wrap(prefix));
        rest
    } else {
        line
    };

    let line = match next_segment(line) {
        None if line.len() > 0 => {
            message.command = Some(wrap(line));
            return Some(message);
        }
        None => return None,
        Some((segment, rest)) => {
            message.command = Some(wrap(segment));
            rest
        }
    };


    // PARAMS
    let mut rest = line;
    while !rest.is_empty() {
        if let Some(':') = rest.chars().next() {
            message.params.push(wrap(&rest[1..]));
            break;
        }

        match next_segment(rest) {
            None => {
                message.params.push(wrap(rest));
                break;
            }
            Some((last, "")) => {
                message.params.push(wrap(last));
                break;
            }
            Some((next, tail)) => {
                message.params.push(wrap(next));
                rest = tail;
            }
        }
    }

    return Some(message);
}
