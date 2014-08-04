use super::parse;


#[test]
fn command_only() {
    let topic = parse("FOO").unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command() {
    let topic = parse(":test FOO").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_trailing_space() {
    let topic = parse(":test FOO  ").unwrap();

    println!("topic: {}", topic.prefix);

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params.len(), 0);
}

#[test]
fn prefix_command_middle_trailing_param() {
    let topic = parse(":test!me@test.ing PRIVMSG #Test :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test!me@test.ing".to_string()));
    assert_eq!(topic.command, Some("PRIVMSG".to_string()));
    assert_eq!(topic.params,
               vec!["#Test".to_string(), "This is a test".to_string()]);
}

#[test]
fn command_middle_trailing_spaces() {
    let topic = parse("PRIVMSG #foo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some("PRIVMSG".to_string()));
    assert_eq!(topic.params,
               vec!["#foo".to_string(), "This is a test".to_string()]);
}

#[test]
fn prefix_command_middle_trailing_spaces() {
    let topic = parse(":test PRIVMSG foo :A string  with spaces   ").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("PRIVMSG".to_string()));
    assert_eq!(topic.params,
               vec!["foo".to_string(), "A string  with spaces   ".to_string()]);
}

#[test]
fn extraneous_spaces() {
    let topic = parse(":test    PRIVMSG  foo   :bar").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("PRIVMSG".to_string()));
    assert_eq!(topic.params,
               vec!["foo".to_string(), "bar".to_string()]);
}

#[test]
fn multiple_params_prefix() {
    let topic = parse(":test FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params,
               vec!["bar".to_string(),
                    "baz".to_string(),
                    "quux".to_string()]);
}

#[test]
fn multiple_middle_no_prefix() {
    let topic = parse("FOO bar baz quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params,
               vec!["bar".to_string(),
                    "baz".to_string(),
                    "quux".to_string()]);
}

#[test]
fn multiple_middle_extra_spaces() {
    let topic = parse("FOO   bar   baz  quux").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params,
               vec!["bar".to_string(),
                    "baz".to_string(),
                    "quux".to_string()]);
}

#[test]
fn multiple_middle_trailing_params() {
    let topic = parse("FOO   bar   baz  quux :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, None);
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params,
               vec!["bar".to_string(),
                    "baz".to_string(),
                    "quux".to_string(),
                    "This is a test".to_string()]);
}

#[test]
fn multiple_middle_containing_colons() {
    let topic = parse(":test PRIVMSG #fo:oo :This is a test").unwrap();
    assert_eq!(topic.tags.len(), 0);
    assert_eq!(topic.prefix, Some("test".to_string()));
    assert_eq!(topic.command, Some("PRIVMSG".to_string()));
    assert_eq!(topic.params,
               vec!["#fo:oo".to_string(),
                    "This is a test".to_string()]);
}

#[test]
fn tags_prefix_command_middle_params_trailiing_params() {
    let topic = parse(
        "@best=super;single :test!me@test.ing FOO bar baz quux :This is a test");
    let topic = topic.unwrap();

    assert_eq!(topic.tags.get(&"best".to_string()), &"super".to_string());
    assert_eq!(topic.tags.get(&"single".to_string()), &"true".to_string());
    assert_eq!(topic.prefix, Some("test!me@test.ing".to_string()));
    assert_eq!(topic.command, Some("FOO".to_string()));
    assert_eq!(topic.params,
               vec!["bar".to_string(),
                    "baz".to_string(),
                    "quux".to_string(),
                    "This is a test".to_string()]);
}
