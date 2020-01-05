use hamberder::lexer;

#[test]
fn test_lexer_empty_source() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    drop(string_tx); // force closed
    for (token, info) in token_rx {
        // bla bla bla
        match token {
            lexer::MaybeToken::Error(_) => {
                assert_eq!(info.line_number, 1);
                assert_eq!(info.char_position, 1);
            }
            _ => assert!(false),
        }
    }
}

#[test]
fn test_lexer_false() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("false")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::FalseLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 5,
            start: 0,
            length: 5
        }
    );
}

#[test]
fn test_lexer_true() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("true")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::TrueLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 4,
            start: 0,
            length: 4
        }
    );
}

#[test]
fn test_lexer_null() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("null")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::NullLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 4,
            start: 0,
            length: 4
        }
    );
}

#[test]
fn test_lexer_chunks() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("nu")).unwrap();
    string_tx.send(String::from("l")).unwrap();
    string_tx.send(String::from("l")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::NullLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 4,
            start: 0,
            length: 4
        }
    );
}

#[test]
fn test_lexer_strip_ws() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("   null ")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::NullLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 7,
            start: 3,
            length: 4
        }
    );
}

#[test]
fn test_lexer_line_nums() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("\n\r\n   null ")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::NullLiteral);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 3,
            char_position: 7,
            start: 6,
            length: 4
        }
    );
}

#[test]
fn test_empty_quoted_string() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("\"\"")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::StringLiteral(String::from("\"\""))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 3,
            start: 0,
            length: 2
        }
    );
}

#[test]
fn test_quoted_string() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("\"abc\"")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::StringLiteral(String::from("\"abc\""))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 6,
            start: 0,
            length: 5
        }
    );
}

#[test]
fn test_illegal_escape_char() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    let test_string = String::from("\"it is illegal to write \\x in json\"");
    string_tx.send(test_string.clone()).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    if let lexer::MaybeToken::Error(_) = &tokens[0].0 {
        //...
    } else {
        assert!(false);
    }
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 26,
            start: 0,
            length: 25
        }
    );
}

#[test]
fn test_lower_uni_chars() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    let test_string = String::from("\"\\uabcd\\uefab\\u01cc\"");
    string_tx.send(test_string.clone()).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::StringLiteral(String::from("\"\\uabcd\\uefab\\u01cc\""))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 21,
            start: 0,
            length: 20
        }
    );
}

#[test]
fn test_upper_uni_chars() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    let test_string = String::from("\"\\uABCD\\uEFAB\\u01CC\"");
    string_tx.send(test_string.clone()).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::StringLiteral(String::from("\"\\uABCD\\uEFAB\\u01CC\""))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 21,
            start: 0,
            length: 20
        }
    );
}

#[test]
fn test_mixed_uni_chars() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    let test_string = String::from("\"\\uAbCD\\uEfaB\\u01cC\"");
    string_tx.send(test_string.clone()).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::StringLiteral(String::from("\"\\uAbCD\\uEfaB\\u01cC\""))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 21,
            start: 0,
            length: 20
        }
    );
}

#[test]
fn test_illegal_uni_char() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    let test_string = String::from("\"\\u0a1H\"");
    string_tx.send(test_string.clone()).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    if let lexer::MaybeToken::Error(_) = &tokens[0].0 {
        //...
    } else {
        assert!(false);
    }
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 7,
            start: 0,
            length: 6
        }
    );
}

#[test]
fn test_multiline_string() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("\"ab\nc\"")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    if let lexer::MaybeToken::Error(_) = &tokens[0].0 {
        //...
    } else {
        assert!(false);
    }
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 4,
            start: 0,
            length: 3
        }
    );
}

#[test]
fn test_single_chars() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("-+{}[].e,:E")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 11);
}

#[test]
fn test_single_zero_ok() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("0")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].0, lexer::MaybeToken::Integer(String::from("0")));
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 2,
            start: 0,
            length: 1
        }
    );
}

/*
move this check to the parser
#[test]
fn test_no_leading_zeroes() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("01234"));
    drop(string_tx); // force closed
    let tokens : Vec<(lexer::MaybeToken,lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    if let lexer::MaybeToken::Error(_) = &tokens[0].0 {
        //...
    } else {
        assert!(false);
    }
    assert_eq!(tokens[0].1, lexer::TokenInfo{line_number: 1, char_position: 1, start: 0, length: 1});
}
*/

#[test]
fn test_parse_bad_float_1() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("- 1234")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].0, lexer::MaybeToken::MinusSign);
    if let lexer::MaybeToken::Error(_) = &tokens[1].0 {
        //...
    } else {
        assert!(false);
    }
}

#[test]
fn test_parse_bad_float_2() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("12\t.\r34")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    println!("dumping tokens: {:?}", tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].0, lexer::MaybeToken::Integer(String::from("12")));
    if let lexer::MaybeToken::Error(_) = &tokens[1].0 {
        //...
    } else {
        assert!(false);
    }
}

#[test]
fn test_integer() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("1234")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].0,
        lexer::MaybeToken::Integer(String::from("1234"))
    );
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 5,
            start: 0,
            length: 4
        }
    );
}

#[test]
fn test_float() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("12.34E+300")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].0, lexer::MaybeToken::Integer(String::from("12")));
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 3,
            start: 0,
            length: 2
        }
    );

    assert_eq!(tokens[1].0, lexer::MaybeToken::Dot);
    assert_eq!(
        tokens[1].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 4,
            start: 2,
            length: 1
        }
    );

    assert_eq!(tokens[2].0, lexer::MaybeToken::Integer(String::from("34")));
    assert_eq!(
        tokens[2].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 6,
            start: 3,
            length: 2
        }
    );

    assert_eq!(tokens[3].0, lexer::MaybeToken::Exponent);
    assert_eq!(
        tokens[3].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 7,
            start: 5,
            length: 1
        }
    );

    assert_eq!(tokens[4].0, lexer::MaybeToken::PlusSign);
    assert_eq!(
        tokens[4].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 8,
            start: 6,
            length: 1
        }
    );

    assert_eq!(tokens[5].0, lexer::MaybeToken::Integer(String::from("300")));
    assert_eq!(
        tokens[5].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 11,
            start: 7,
            length: 3
        }
    );
}

#[test]
fn test_lexer_empty_array() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("[]")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].0, lexer::MaybeToken::LeftBracket);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 2,
            start: 0,
            length: 1
        }
    );
    assert_eq!(tokens[1].0, lexer::MaybeToken::RightBracket);
    assert_eq!(
        tokens[1].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 3,
            start: 1,
            length: 1
        }
    );
}

#[test]
fn test_lexer_array() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx
        .send(String::from("[false, null, true, 1.234E+2, \"bla\", {}]"))
        .unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 19);
    assert_eq!(tokens[0].0, lexer::MaybeToken::LeftBracket);
    assert_eq!(tokens[1].0, lexer::MaybeToken::FalseLiteral);
    assert_eq!(tokens[2].0, lexer::MaybeToken::Comma);
    assert_eq!(tokens[3].0, lexer::MaybeToken::NullLiteral);
    assert_eq!(tokens[4].0, lexer::MaybeToken::Comma);
    assert_eq!(tokens[5].0, lexer::MaybeToken::TrueLiteral);
    assert_eq!(tokens[6].0, lexer::MaybeToken::Comma);
    assert_eq!(tokens[7].0, lexer::MaybeToken::Integer(String::from("1")));
    assert_eq!(tokens[8].0, lexer::MaybeToken::Dot);
    assert_eq!(tokens[9].0, lexer::MaybeToken::Integer(String::from("234")));
    assert_eq!(tokens[10].0, lexer::MaybeToken::Exponent);
    assert_eq!(tokens[11].0, lexer::MaybeToken::PlusSign);
    assert_eq!(tokens[12].0, lexer::MaybeToken::Integer(String::from("2")));
    assert_eq!(tokens[13].0, lexer::MaybeToken::Comma);
    assert_eq!(
        tokens[14].0,
        lexer::MaybeToken::StringLiteral(String::from("\"bla\""))
    );
    assert_eq!(tokens[15].0, lexer::MaybeToken::Comma);
    assert_eq!(tokens[16].0, lexer::MaybeToken::LeftCurly);
    assert_eq!(tokens[17].0, lexer::MaybeToken::RightCurly);
    assert_eq!(tokens[18].0, lexer::MaybeToken::RightBracket);
}

#[test]
fn test_lexer_empty_object() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(String::from("{}")).unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].0, lexer::MaybeToken::LeftCurly);
    assert_eq!(
        tokens[0].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 2,
            start: 0,
            length: 1
        }
    );
    assert_eq!(tokens[1].0, lexer::MaybeToken::RightCurly);
    assert_eq!(
        tokens[1].1,
        lexer::TokenInfo {
            line_number: 1,
            char_position: 3,
            start: 1,
            length: 1
        }
    );
}

/*
#[test]
fn test_lexer_finds_next_line() {
let test_str = String::from("[{\"id\":1,\"first_name\":\"Audy\",\"last_name\":\"Taborre\",\"lat\":-17.3058881,\"long\":31.5655424},
{");
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx.send(test_str).unwrap();
    drop(string_tx);
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    println!("tok dump: {:?}", tokens);
    assert_eq!(tokens.len(), 29);
}
*/

#[test]
fn test_lexer_object() {
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    lexer::lex(string_rx, token_tx);
    string_tx
        .send(String::from(
            "{
        \"key1\": null,
        \"key2\": -42,
        \"key3\": [
            false
        ], 
        \"key4\": {
            \"key5\": {}
        }
    }",
        ))
        .unwrap();
    drop(string_tx); // force closed
    let tokens: Vec<(lexer::MaybeToken, lexer::TokenInfo)> = token_rx.iter().collect();
    assert_eq!(tokens.len(), 25);
    let expected_tokens = vec![
        lexer::MaybeToken::LeftCurly,
        lexer::MaybeToken::StringLiteral(String::from("\"key1\"")),
        lexer::MaybeToken::Colon,
        lexer::MaybeToken::NullLiteral,
        lexer::MaybeToken::Comma,
        lexer::MaybeToken::StringLiteral(String::from("\"key2\"")),
        lexer::MaybeToken::Colon,
        lexer::MaybeToken::MinusSign,
        lexer::MaybeToken::Integer(String::from("42")),
        lexer::MaybeToken::Comma,
        lexer::MaybeToken::StringLiteral(String::from("\"key3\"")),
        lexer::MaybeToken::Colon,
        lexer::MaybeToken::LeftBracket,
        lexer::MaybeToken::FalseLiteral,
        lexer::MaybeToken::RightBracket,
        lexer::MaybeToken::Comma,
        lexer::MaybeToken::StringLiteral(String::from("\"key4\"")),
        lexer::MaybeToken::Colon,
        lexer::MaybeToken::LeftCurly,
        lexer::MaybeToken::StringLiteral(String::from("\"key5\"")),
        lexer::MaybeToken::Colon,
        lexer::MaybeToken::LeftCurly,
        lexer::MaybeToken::RightCurly,
        lexer::MaybeToken::RightCurly,
        lexer::MaybeToken::RightCurly,
    ];
    for (i, (tok, _)) in tokens.iter().enumerate() {
        assert_eq!(&expected_tokens[i], tok);
    }
}
