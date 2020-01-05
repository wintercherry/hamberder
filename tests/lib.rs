pub mod test_lexer;
pub mod test_parser;

use hamberder::*;

#[test]
fn test_empty() {
    let test_str = String::from("");
    let (str_tx, str_rx) = std::sync::mpsc::channel();
    let tag_rx = hamberder::parse(str_rx);
    str_tx.send(test_str).unwrap();
    drop(str_tx);
    let tags: parser::TagVec = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);

}

#[test]
fn test_obj() {
    let test_str = String::from("
{
    \"version\":  1.0,
    \"config\": [\"en\", \"de\", \"jp\"],
    \"disabled\": true
}
    ");
    let (str_tx, str_rx) = std::sync::mpsc::channel();
    let tag_rx = hamberder::parse(str_rx);
    str_tx.send(test_str).unwrap();
    drop(str_tx);
    let tags: parser::TagVec = tag_rx.iter().collect();
    assert_eq!(tags.len(), 12);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    assert_eq!(tags[1], parser::Tag::ObjectKey(String::from("version")));
    assert_eq!(tags[2], parser::Tag::Number(String::from("1.0")));
    assert_eq!(tags[3], parser::Tag::ObjectKey(String::from("config")));
    assert_eq!(tags[4], parser::Tag::BeginArray);
    assert_eq!(tags[5], parser::Tag::StringLiteral(String::from("en")));
    assert_eq!(tags[6], parser::Tag::StringLiteral(String::from("de")));
    assert_eq!(tags[7], parser::Tag::StringLiteral(String::from("jp")));
    assert_eq!(tags[8], parser::Tag::EndArray);
    assert_eq!(tags[9], parser::Tag::ObjectKey(String::from("disabled")));
    assert_eq!(tags[10], parser::Tag::TrueLiteral);
    assert_eq!(tags[11], parser::Tag::EndObject);
}