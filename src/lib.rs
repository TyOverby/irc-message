#![allow(dead_code)]
use std::collections::hashmap::HashMap;

#[cfg(test)]
mod tests;

pub struct IrcMessage {
    pub tags: HashMap<String, String>,
    pub prefix: Option<String>,
    pub command: Option<String>,
    pub params: Vec<String>
}

impl IrcMessage {
    pub fn new_empty() -> IrcMessage {
        IrcMessage {
            tags: HashMap::new(),
            prefix: None,
            command: None,
            params: Vec::new()
        }
    }
}

fn find_after_n(string: &str, c: char, n: uint) -> Option<uint> {
    string.slice_from(n).find(c).map(|a| a + n)
}

fn is_white(c: &char) -> bool { *c == ' ' }
fn not_white(c: &char) -> bool { !is_white(c) }

pub fn parse(line: &str) -> Result<IrcMessage, ()> {
    let mut message = IrcMessage::new_empty();
    let mut chars = line.chars().peekable();
    // Tags
    if chars.peek() == Some(&'@') {
        let mut tags = chars.by_ref().skip(1).take_while(not_white);
        loop {
            let mut tag = tags.by_ref().take_while(|&c| c != ';').peekable();
            if tag.peek() == None { break; }
            let name = tag.by_ref().take_while(|&c| c != '=').collect();
            let mut rest = tag.peekable();
            let value = if rest.is_empty() {
                "true".to_string()
            } else {
                rest.collect()
            };
            message.tags.insert(name, value);
        }
    }

    let mut chars = chars.skip_while(is_white).peekable();

    // Prefix
    if chars.peek() == Some(&':') {
        let prefix = chars.by_ref().skip(1).take_while(not_white).collect();
        message.prefix = Some(prefix);
    }

    let mut chars = chars.skip_while(is_white);

    // Command
    let command = chars.by_ref().take_while(not_white).collect();
    message.command = Some(command);

    let mut chars = chars.skip_while(is_white);

    // Params
    loop {
        let mut chars = chars.by_ref().skip_while(is_white).peekable();
        match chars.peek() {
            None => {
                println!("breaking on none");
                break; }
            Some(&':') => {
                message.params.push(chars.skip(1).collect());
                println!("breaking on done");
                break;
            }
            _ => {
                println!("\nbefore");
                chars.by_ref().inspect(|&c| print!("{}", c) );
                message.params.push(chars.by_ref().take_while(not_white).collect());
            }
        }
    }

    Ok(message)
}
/*
pub fn parse2(line: &str) -> Result<IrcMessage, ()> {
    let mut message = IrcMessage::new_empty();

    let mut position = 0;

    if line.char_at(0) == '@' {
        let next_space = match line.find(' ') {
            Some(a) => a,
            None => return Err(())
        };

        let mut rawTags = line.slice(1, next_space).split(';');
        for tag in rawTags {
            if tag.contains_char('=') {
                let mut pair = tag.split('=');
                message.tags.insert(pair.next().unwrap().to_string(), pair.next().unwrap().to_string());
            } else {
                message.tags.insert(tag.to_string(), "true".to_string());
            }
        }
        position = next_space + 1;

    }

    // Eat whitespace
    while line.char_at(position) == ' ' {
        position += 1;
    }

    // look for prefix
    if line.char_at(position) == ':' {
        let next_space = match find_after_n(line, ' ', position) {
            Some(a) => a,
            None => return Err(())
        };
        message.prefix = Some(
            line.slice(position + 1, next_space).to_string());
        position = next_space + 1;
        while position < line.len() && line.char_at(position) == ' ' {
            position += 1;
        }
    }

    let next_space = match find_after_n(line, ' ', position) {
        Some(a) => a,
        None => {
            if line.len() > position {
                message.command = Some(line.slice_from(position).to_string());
            }
            return Ok(message);
        }
    };

    message.command = Some(line.slice(position, next_space).to_string());

    position = next_space + 1;

    while position < line.len() && line.char_at(position) == ' ' {
        position += 1;
    }

    while position < line.len() {
        let next_space = find_after_n(line, ' ', position);

        if line.char_at(position) == ':' {
            message.params.push(line.slice_from(position +1).to_string());
            break;
        }

        match next_space {
            Some(n) => {
                message.params.push(line.slice(position, n).to_string());
                position = n + 1;
                while line.char_at(position) == ' ' {
                    position += 1;
                }
                continue;
            },
            None => {
                message.params.push(line.slice_from(position).to_string());
                break;
            }
        }
    }

    return Ok(message);
}
*/
