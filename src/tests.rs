use super::parse_slice;
use std::str::Slice;

#[test]
fn command_only() {
    let topic = parse_slice("FOO").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command() {
    let topic = parse_slice(":test FOO").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_trailing_space() {
    let topic = parse_slice(":test FOO  ").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_middle_trailing_param() {
    let topic = parse_slice(":test!me@test.ing PRIVMSG #Test :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test!me@test.ing")));
    assert_eq!(topic.command, Some(Slice("PRIVMSG")));
    assert_eq!(topic.params, vec![Slice("#Test"), Slice("This is a test")]);
}

#[test]
fn command_middle_trailing_spaces() {
    let topic = parse_slice("PRIVMSG #foo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Slice("PRIVMSG")));
    assert_eq!(topic.params, vec![Slice("#foo"), Slice("This is a test")]);
}

#[test]
fn prefix_command_middle_trailing_spaces() {
    let topic = parse_slice(":test PRIVMSG foo :A string  with spaces   ").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("PRIVMSG")));
    assert_eq!(topic.params, vec![Slice("foo"), Slice("A string  with spaces   ")]);
}

#[test]
fn extraneous_spaces() {
    let topic = parse_slice(":test    PRIVMSG  foo   :bar").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("PRIVMSG")));
    assert_eq!(topic.params, vec![Slice("foo"), Slice("bar")]);
}

#[test]
fn multiple_params_prefix() {
    let topic = parse_slice(":test FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params, vec![Slice("bar"), Slice("baz"), Slice("quux")]);
}

#[test]
fn multiple_middle_no_prefix() {
    let topic = parse_slice("FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params, vec![Slice("bar"), Slice("baz"), Slice("quux")]);
}

#[test]
fn multiple_middle_extra_spaces() {
    let topic = parse_slice("FOO   bar   baz  quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params, vec![Slice("bar"), Slice("baz"), Slice("quux")]);
}

#[test]
fn multiple_middle_trailing_params() {
    let topic = parse_slice("FOO   bar   baz  quux :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params, vec![Slice("bar"), Slice("baz"), Slice("quux"), Slice("This is a test")]);
}

#[test]
fn multiple_middle_containing_colons() {
    let topic = parse_slice(":test PRIVMSG #fo:oo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some(Slice("test")));
    assert_eq!(topic.command, Some(Slice("PRIVMSG")));
    assert_eq!(topic.params,
               vec![Slice("#fo:oo"),
                    Slice("This is a test")]);
}

#[test]
fn tags_prefix_command_middle_params_trailiing_params() {
    let topic = parse_slice(
        "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.get(&Slice("best")), &Slice("super"));
    assert_eq!(topic.tags.get(&Slice("single")), &Slice("true"));
    assert_eq!(topic.prefix, Some(Slice("test!me@test.ing")));
    assert_eq!(topic.command, Some(Slice("FOO")));
    assert_eq!(topic.params,
               vec![Slice("bar"),
                    Slice("baz"),
                    Slice("quux"),
                    Slice("This is a test")]);
}
