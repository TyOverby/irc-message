use super::IrcMessage;

#[test]
fn command_only() {
    let topic: IrcMessage<&'static str> = IrcMessage::parse_ref("FOO").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test FOO").unwrap();

    println!("topic: {:?}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_trailing_space() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test FOO  ").unwrap();

    println!("topic: {:?}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_middle_trailing_param() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test!me@test.ing PRIVMSG #Test :This is a test").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test!me@test.ing")));
    assert_eq!(topic.command, Some(("PRIVMSG")));
    assert_eq!(topic.params, vec![("#Test"), ("This is a test")]);
}

#[test]
fn command_middle_trailing_spaces() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref("PRIVMSG #foo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(("PRIVMSG")));
    assert_eq!(topic.params, vec![("#foo"), ("This is a test")]);
}

#[test]
fn prefix_command_middle_trailing_spaces() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test PRIVMSG foo :A string  with spaces   ").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("PRIVMSG")));
    assert_eq!(topic.params, vec![("foo"), ("A string  with spaces   ")]);
}

#[test]
fn extraneous_spaces() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test    PRIVMSG  foo   :bar").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("PRIVMSG")));
    assert_eq!(topic.params, vec![("foo"), ("bar")]);
}

#[test]
fn multiple_params_prefix() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params, vec![("bar"), ("baz"), ("quux")]);
}

#[test]
fn multiple_middle_no_prefix() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref("FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params, vec![("bar"), ("baz"), ("quux")]);
}

#[test]
fn multiple_middle_extra_spaces() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref("FOO   bar   baz  quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params, vec![("bar"), ("baz"), ("quux")]);
}

#[test]
fn multiple_middle_trailing_params() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref("FOO   bar   baz  quux :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(("FOO")));
    assert_eq!(topic.params, vec![("bar"), ("baz"), ("quux"), ("This is a test")]);
}

#[test]
fn multiple_middle_containing_colons() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(":test PRIVMSG #fo:oo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(("test")));
    assert_eq!(topic.command, Some(("PRIVMSG")));
    assert_eq!(topic.params,
               vec![("#fo:oo"),
                    ("This is a test")]);
}

#[test]
fn tags_prefix_command_middle_params_trailiing_params() {
     let topic: IrcMessage<&'static str> = IrcMessage::parse_ref(
        "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test")
         .unwrap();

    assert!(topic.tags[("best")] == ("super"));
    assert!(topic.tags[("single")] == ("true"));
    assert!(topic.prefix == Some(("test!me@test.ing")));
    assert!(topic.command == Some(("FOO")));
    assert!(topic.params ==
               vec![("bar"),
                    ("baz"),
                    ("quux"),
                    ("This is a test")]);
}

#[cfg(test)]
fn parse_file(filepath: &str) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(filepath).unwrap();
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line.unwrap();
        assert!(IrcMessage::parse_ref(&line[..]).is_some());
    }
}

#[test]
fn read_intro_logs_1() {
    parse_file("./examples/intro.txt");
}

#[test]
fn read_intro_logs_2() {
    parse_file("./examples/intro2.txt");
}

#[test]
fn read_long_logs_1() {
    parse_file("./examples/long.txt");
}

#[test]
fn read_long_logs_2() {
    parse_file("./examples/long2.txt");
}
