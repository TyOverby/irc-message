use super::{IrcMessage, StrCow};
use std::borrow::Cow;

#[test]
fn command_only() {
    let topic = IrcMessage::parse_borrowed("FOO").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command() {
    let topic = IrcMessage::parse_borrowed(":test FOO").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_trailing_space() {
    let topic = IrcMessage::parse_borrowed(":test FOO  ").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_middle_trailing_param() {
    let topic = IrcMessage::parse_borrowed(":test!me@test.ing PRIVMSG #Test :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test!me@test.ing")));
    assert_eq!(topic.command, Some(Cow::Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Cow::Borrowed("#Test"), Cow::Borrowed("This is a test")]);
}

#[test]
fn command_middle_trailing_spaces() {
    let topic = IrcMessage::parse_borrowed("PRIVMSG #foo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Cow::Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Cow::Borrowed("#foo"), Cow::Borrowed("This is a test")]);
}

#[test]
fn prefix_command_middle_trailing_spaces() {
    let topic = IrcMessage::parse_borrowed(":test PRIVMSG foo :A string  with spaces   ").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Cow::Borrowed("foo"), Cow::Borrowed("A string  with spaces   ")]);
}

#[test]
fn extraneous_spaces() {
    let topic = IrcMessage::parse_borrowed(":test    PRIVMSG  foo   :bar").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("PRIVMSG")));
    assert_eq!(topic.params, vec![Cow::Borrowed("foo"), Cow::Borrowed("bar")]);
}

#[test]
fn multiple_params_prefix() {
    let topic = IrcMessage::parse_borrowed(":test FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params, vec![Cow::Borrowed("bar"), Cow::Borrowed("baz"), Cow::Borrowed("quux")]);
}

#[test]
fn multiple_middle_no_prefix() {
    let topic = IrcMessage::parse_borrowed("FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params, vec![Cow::Borrowed("bar"), Cow::Borrowed("baz"), Cow::Borrowed("quux")]);
}

#[test]
fn multiple_middle_extra_spaces() {
    let topic = IrcMessage::parse_borrowed("FOO   bar   baz  quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params, vec![Cow::Borrowed("bar"), Cow::Borrowed("baz"), Cow::Borrowed("quux")]);
}

#[test]
fn multiple_middle_trailing_params() {
    let topic = IrcMessage::parse_borrowed("FOO   bar   baz  quux :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params, vec![Cow::Borrowed("bar"), Cow::Borrowed("baz"), Cow::Borrowed("quux"), Cow::Borrowed("This is a test")]);
}

#[test]
fn multiple_middle_containing_colons() {
    let topic = IrcMessage::parse_borrowed(":test PRIVMSG #fo:oo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test")));
    assert_eq!(topic.command, Some(Cow::Borrowed("PRIVMSG")));
    assert_eq!(topic.params,
               vec![Cow::Borrowed("#fo:oo"),
                    Cow::Borrowed("This is a test")]);
}

#[test]
fn tags_prefix_command_middle_params_trailiing_params() {
    let topic = IrcMessage::parse_borrowed(
        "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags[Cow::Borrowed("best")], Cow::Borrowed("super"));
    assert_eq!(topic.tags[Cow::Borrowed("single")], Cow::Borrowed("true"));
    assert_eq!(topic.prefix, Some(Cow::Borrowed("test!me@test.ing")));
    assert_eq!(topic.command, Some(Cow::Borrowed("FOO")));
    assert_eq!(topic.params,
               vec![Cow::Borrowed("bar"),
                    Cow::Borrowed("baz"),
                    Cow::Borrowed("quux"),
                    Cow::Borrowed("This is a test")]);
}

#[cfg(test)]
fn parse_file(filepath: &str) {
    use std::io::fs::File;
    use std::io::BufferedReader;
    let file = File::open(&Path::new(filepath)).unwrap();
    let mut file = BufferedReader::new(file);
    for line in file.lines() {
        let line = line.unwrap();
        assert!(IrcMessage::parse_borrowed(line.as_slice()).is_ok());
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
