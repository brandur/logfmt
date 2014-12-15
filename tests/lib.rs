extern crate logfmt;

fn pair<'a>(key: &'a str, val: Option<&'a str>) -> logfmt::Pair<'a> {
    match val {
        Some(v) =>
            logfmt::Pair { key: key, val: Some(v) },
        None =>
            logfmt::Pair { key: key, val: None },
    }
}

#[test]
fn it_parses() {
    assert_eq!(vec![
        pair("a", Some("1")),
        pair("b", Some("bar")),
        pair("ƒ", Some("2h3s")),
        pair("r", Some("esc\t")),
        pair("d", None),
        pair("x", Some("sf"))
    ], logfmt::parse("a=1 b=\"bar\" ƒ=2h3s r=\"esc\t\" d x=sf"));

    assert_eq!(vec![
        pair("x", Some(""))
    ], logfmt::parse("x= "));

    assert_eq!(vec![
        pair("y", Some(""))
    ], logfmt::parse("y="));

    assert_eq!(vec![
        pair("y", None)
    ], logfmt::parse("y"));

    assert_eq!(vec![
        pair("y", None)
    ], logfmt::parse("y"));

    assert_eq!(vec![
        pair("y", Some("f"))
    ], logfmt::parse("y=f"));

    assert_eq!(vec![
        pair("y", Some("f"))
    ], logfmt::parse("y=\"f\""));

    assert_eq!(vec![
        pair("y", Some("f(\"x\")"))
    ], logfmt::parse("y=\"f(\\\"x\\\")"));

    // unknown escapes just get written to value
    assert_eq!(vec![
        pair("y", Some("\\x"))
    ], logfmt::parse("y=\\x"));

    // this is considered garbage and produces nothing
    assert_eq!(vec![
    ], logfmt::parse("=y"));
}
