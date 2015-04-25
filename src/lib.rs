#![allow(dead_code)]
#![feature(macro_rules)]
use std::collections::hash_map::HashMap;
use std::borrow::Cow;

#[cfg(test)]
mod tests;

pub type StrCow<'a> = Cow<'a, String, str>;

macro_rules! try_o {
    (a) => {
        (match a {
            Some(x) => x,
            None => return None
        })
    }
}

#[derive(Show)]
pub struct IrcMessage<'a> {
    pub tags: HashMap<StrCow<'a>, StrCow<'a>>,
    pub prefix: Option<StrCow<'a>>,
    pub command: Option<StrCow<'a>>,
    pub params: Vec<StrCow<'a>>
}

impl <'b> IrcMessage<'b> {
    pub fn new_empty<'a>() -> IrcMessage<'a> {
        IrcMessage {
            tags: HashMap::new(),
            prefix: None,
            command: None,
            params: Vec::new()
        }
    }

    /// Parse a message from a string to an owned `IrcMessage`.
    pub fn parse_owned<'a>(s: &'a str) -> Result<IrcMessage<'static>, ()> {
        return parse_owned(s);
    }
    /// Parse a message from a string to an `IrcMessage` that still refers
    /// to the original `str`.  Useful for minimizing allocations.
    pub fn parse_borrowed<'a>(s: &'a str) -> Result<IrcMessage<'a>, ()> {
        return parse_borrowed(s);
    }
}

fn next_segment<'a>(line: &'a str) -> Option<(&'a str, &'a str)> {
    match line.find(' ') {
        Some(n) => {
            if line.len() == 0 {
                return (line, line);
            }
            let segment = line.slice_to(n);

            let mut p = n;

            while line.char_at(p) == ' ' && p < line.len() - 1 {
                p += 1;
            }

            (segment, line.slice_from(p))
        },
        None => (line, "")
    }
}

fn trim_space<'a>(line: &'a str) -> &'a str {
    if line.len() == 0 {
        return line;
    }

    let mut p = 0;
    while line.char_at(p) == ' ' && p < line.len() - 1 {
        p += 1;
    }

    line.slice_from(p)
}

fn parse_owned<'a>(line: &'a str) -> Result<IrcMessage<'static>, ()> {
    parse_into(line, |a| Cow::Owned(a.to_string()))
}

fn parse_borrowed<'a>(line: &'a str) -> Result<IrcMessage<'a>, ()> {
    parse_into(line, Cow::Borrowed)
}

fn parse_into<'a, 'b, F>(line: &'a str, wrap: F) -> Option<IrcMessage<'b>>
where F: Fn(&'a str) -> StrCow<'b> {
    let mut message = IrcMessage::new_empty();

    // TAGS
    let line = if line.char_at(0) == '@' {
        let (tags, rest) = try_o!(next_segment(line.slice_from(1)));
        let mut raw_tags = tags.split(';');
        for tag in raw_tags {
            if tag.contains_char('=') {
                let mut pair = tag.split('=');
                message.tags.insert(wrap(try_o!(pair.next())),
                                    wrap(try_o!(pair.next())));
            } else {
                message.tags.insert(wrap(tag), wrap("true"));
            }
        }
        rest
    } else {
        line
    };

    // PREFIX
    let line = if line.char_at(0) == ':' {
        let (prefix, rest) = try_o!(next_segment(line.slice_from(1)));
        message.prefix = Some(wrap(prefix));
        rest
    } else {
        line
    };

    // COMMAND
    let (command, line) = next_segment(line);
    message.command = Some(wrap(command));


    // PARAMS
    let mut rest = trim_space(line);
    while !rest.is_empty() {
        match next_segment(rest) {
            ("", _) => {
                break;
            }
            (_, _) if rest.char_at(0) == ':' => {
                message.params.push(wrap(rest.slice_from(1)));
                break;
            }
            (last, "") => {
                message.params.push(wrap(last));
                break;
            }
            (next, tail) => {
                message.params.push(wrap(next));
                rest = tail;
            }
        }
    }

    return Ok(message);
}
