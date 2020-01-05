use hamberder::{lexer, parser};
//use std::sync::mpsc;

#[test]
fn test_parse_error() {
    let err_info = lexer::ErrorInfo {
        message: String::from("bla bla bla"),
        fragment: None,
    };
    let dummy_token = lexer::MaybeToken::Error(err_info.clone());
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Error(err_info, dummy_token_info));
}

#[test]
fn test_parse_true() {
    let dummy_token = lexer::MaybeToken::TrueLiteral;
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::TrueLiteral);
}

#[test]
fn test_parse_false() {
    let dummy_token = lexer::MaybeToken::FalseLiteral;
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::FalseLiteral);
}

#[test]
fn test_parse_null() {
    let dummy_token = lexer::MaybeToken::NullLiteral;
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::NullLiteral);
}

#[test]
fn test_parse_string() {
    let dummy_token = lexer::MaybeToken::StringLiteral(String::from("\"abc\""));
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::StringLiteral(String::from("abc")));
}

#[test]
fn test_parse_empty_string() {
    let dummy_token = lexer::MaybeToken::StringLiteral(String::from("\"\""));
    let dummy_token_info = lexer::TokenInfo {
        line_number: 1,
        char_position: 3,
        start: 0,
        length: 2,
    };
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((dummy_token, dummy_token_info.clone()))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::StringLiteral(String::from("")));
}

#[test]
fn test_parse_float() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::MinusSign,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("3")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 3,
                start: 1,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Dot,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 4,
                start: 2,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("14159")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 9,
                start: 3,
                length: 5,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Number(String::from("-3.14159")));
}

#[test]
fn test_parse_int() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("37728")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 0,
                length: 5,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Number(String::from("37728")));
}

#[test]
fn test_parse_exp() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("37728")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 0,
                length: 5,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Exponent,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("117")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 10,
                start: 6,
                length: 3,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Number(String::from("37728E117")));
}

#[test]
fn test_parse_exp_with_plus() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("37728")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 0,
                length: 5,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Exponent,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::PlusSign,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 6,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("117")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 11,
                start: 7,
                length: 3,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Number(String::from("37728E+117")));
}

#[test]
fn test_parse_exp_with_minus() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("37728")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 0,
                length: 5,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Exponent,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::MinusSign,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 6,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Integer(String::from("117")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 11,
                start: 7,
                length: 3,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0], parser::Tag::Number(String::from("37728E-117")));
}

#[test]
fn test_parse_empty_object() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 3,
                start: 1,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    assert_eq!(tags[1], parser::Tag::EndObject);
}

#[test]
fn test_parse_object_no_key_name() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::TrueLiteral,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 1,
                length: 4,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    if let parser::Tag::Error(_, _) = tags[1] {
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_object_no_colon() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::StringLiteral(String::from("\"blab\"")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 1,
                length: 4,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    assert_eq!(tags[1], parser::Tag::ObjectKey(String::from("blab")));
    if let parser::Tag::Error(_, _) = tags[2] {
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_object_no_value() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::StringLiteral(String::from("\"blab\"")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 1,
                length: 4,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Colon,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 6,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    println!("dumping tags: {:?}", tags);
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    assert_eq!(tags[1], parser::Tag::ObjectKey(String::from("blab")));
    if let parser::Tag::Error(_, _) = tags[2] {
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_object_double_comma() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::StringLiteral(String::from("\"blab\"")),
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 1,
                length: 4,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Colon,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::NullLiteral,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 11,
                start: 6,
                length: 4,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Comma,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 10,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Comma,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 9,
                start: 11,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 6,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 4);
    assert_eq!(tags[0], parser::Tag::BeginObject);
    assert_eq!(tags[1], parser::Tag::ObjectKey(String::from("blab")));
    assert_eq!(tags[2], parser::Tag::NullLiteral);
    if let parser::Tag::Error(_, _) = tags[3] {
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_empty_array() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 3,
                start: 1,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], parser::Tag::BeginArray);
    assert_eq!(tags[1], parser::Tag::EndArray);
}

#[test]
fn test_parse_array_stray_comma() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::Comma,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 3,
                start: 1,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::RightBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 4,
                start: 2,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], parser::Tag::BeginArray);
    if let parser::Tag::Error(_, _) = tags[1] {
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_array() {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    parser::parse(token_rx, tag_tx);
    token_tx
        .send((
            lexer::MaybeToken::LeftBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 2,
                start: 0,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::FalseLiteral,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 6,
                start: 1,
                length: 4,
            },
        ))
        .unwrap();        
    token_tx
        .send((
            lexer::MaybeToken::Comma,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 7,
                start: 5,
                length: 1,
            },
        ))
        .unwrap();
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 8,
                start: 6,
                length: 1,
            },
        ))
        .unwrap(); 
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 9,
                start: 7,
                length: 1,
            },
        ))
        .unwrap(); 
    token_tx
        .send((
            lexer::MaybeToken::Comma,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 10,
                start: 8,
                length: 1,
            },
        ))
        .unwrap(); 
    token_tx
        .send((
            lexer::MaybeToken::LeftCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 11,
                start: 9,
                length: 1,
            },
        ))
        .unwrap(); 
    token_tx
        .send((
            lexer::MaybeToken::RightCurly,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 12,
                start: 10,
                length: 1,
            },
        ))
        .unwrap(); 
    token_tx
        .send((
            lexer::MaybeToken::RightBracket,
            lexer::TokenInfo {
                line_number: 1,
                char_position: 13,
                start: 11,
                length: 1,
            },
        ))
        .unwrap();
    drop(token_tx);
    let tags: Vec<parser::Tag> = tag_rx.iter().collect();
    println!("dump toks {:?}", tags);
    assert_eq!(tags.len(), 7);
    assert_eq!(tags[0], parser::Tag::BeginArray);
    assert_eq!(tags[1], parser::Tag::FalseLiteral);
    assert_eq!(tags[2], parser::Tag::BeginObject);
    assert_eq!(tags[3], parser::Tag::EndObject);
    assert_eq!(tags[4], parser::Tag::BeginObject);
    assert_eq!(tags[5], parser::Tag::EndObject);
    assert_eq!(tags[6], parser::Tag::EndArray);
}