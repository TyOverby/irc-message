#![allow(dead_code)]
use std::collections::hashmap::HashMap;
use std::vec::MoveItems;

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

// Strips whitespace off an iterator.  It also recollects the
// characters in order to keep rustc from blowing itself up.
// github.com/rust-lang/rust/issues/16232
fn strip<I: Iterator<char>>(iter: I) -> MoveItems<char> {
    let vec: Vec<char> = iter.skip_while(is_white).collect();
    vec.move_iter()
}

pub fn parse(line: &str) -> Result<IrcMessage, ()> {
    let mut message = IrcMessage::new_empty();
    let mut chars = line.chars().peekable();
    // Tags
    if chars.peek() == Some(&'@') {
        // Tags are from the @ to the first space character.
        let mut tags = chars.by_ref().skip(1).take_while(not_white);
        loop {
            // Tags are seperated by semicolons
            let mut tag = tags.by_ref().take_while(|&c| c != ';').peekable();
            if tag.peek() == None { break; }
            // A tag might have a '=' seperating key and value pairs
            let name = tag.by_ref().take_while(|&c| c != '=').collect();
            let mut rest = tag.peekable();
            let value = if rest.is_empty() {
                // True is the default value if one is not provided.
                "true".to_string()
            } else {
                rest.collect()
            };
            message.tags.insert(name, value);
        }
    }

    // An empty iterator at this point is a parsing failure
    let mut chars = strip(chars).peekable();
    if chars.is_empty() { return Err(()); }

    // Prefix
    if chars.peek() == Some(&':') {
        let prefix = chars.by_ref().skip(1).take_while(not_white).collect();
        message.prefix = Some(prefix);
    }

    let mut chars = strip(chars).peekable();
    if chars.is_empty() { return Err(()) }

    // Command
    let command = chars.by_ref().take_while(not_white).collect();
    message.command = Some(command);
    let mut chars = strip(chars).peekable();

    // Params
    loop {
        match chars.peek() {
            None => { break; }
            Some(&':') => {
                // The rest of the string
                message.params.push(chars.skip(1).collect());
                break;
            }
            _ => {
                message.params.push(chars.by_ref().take_while(not_white).collect());
                chars = strip(chars).peekable();
            }
        }
    }

    Ok(message)
}
