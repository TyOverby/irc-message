extern crate irc_message;

use irc_message::IrcMessage;

fn main() {
    let m = "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test";
    let parsed = IrcMessage::parse_slice(m);
    println!("{}", parsed);
}
