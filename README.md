irc-message
===========

irc-message is an IRC message parsing library for rust.

## Example

```
extern crate irc_message;

use irc_message::IrcMessage;

fn main() {
    let m = "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test";
    let parsed = IrcMessage::parse_slice(m);
    println!("{}", parsed);
}
```

### Result

```
Ok(
  IrcMessage { 
    tags: {
      best: super, 
      single: true
    }, 
    prefix: Some(test!me@test.ing), 
    command: Some(FOO), 
    params: [bar, baz, quux, This is a test] 
  }
)
```
