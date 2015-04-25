use std::collections::hash_map::HashMap;
use std::hash::Hash;

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

#[derive(Debug)]
pub struct IrcMessage<T: Eq + Hash> {
    pub tags: HashMap<T, T>,
    pub prefix: Option<T>,
    pub command: Option<T>,
    pub params: Vec<T>
}

impl IrcMessage<String> {
    pub fn parse_own(input: &str) -> Option<IrcMessage<String>> {
        parse_owned(input)
    }
}

impl <'a> IrcMessage<&'a str> {
    pub fn parse_ref(input: &'a str) -> Option<IrcMessage<&'a str>> {
        parse_slice(input)
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

fn parse_owned<'a>(line: &'a str) -> Option<IrcMessage<String>> {
    parse_into(line, |a| a.to_string())
}

fn parse_slice<'a>(line: &'a str) -> Option<IrcMessage<&'a str>> {
    parse_into(line, |a| a)
}

fn parse_into<'a, T: Eq + Hash, F>(line: &'a str, wrap: F) -> Option<IrcMessage<T>>
where F: Fn(&'a str) -> T {
    let mut message = IrcMessage {
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
    let line = if try_o!(line.chars().next()) == ':' {
        let (prefix, rest) = try_o!(next_segment(&line[1..]));
        message.prefix = Some(wrap(prefix));
        rest
    } else {
        line
    };

    // COMMAND
    /*let (command, line) = try_o!(next_segment(line));
    message.command = Some(wrap(command));
    */

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
