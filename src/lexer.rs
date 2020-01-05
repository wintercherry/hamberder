use std::{sync::mpsc, thread};

#[derive(PartialEq, std::fmt::Debug, Clone)]
pub struct TokenInfo {
    pub line_number: usize, // line number (if any) of the stream. starts with 1, not 0
    pub char_position: usize, // *end* character/column position relative to line_number. maybe useful for error messages. starts with 1, not 0
    pub start: usize,         // absolute starting character position
    pub length: usize,        // length of token in stream
}

#[derive(PartialEq, std::fmt::Debug, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub fragment: Option<String>,
}

#[derive(PartialEq, std::fmt::Debug)]
pub enum MaybeToken {
    // todo: list all valid tokens
    FalseLiteral,
    TrueLiteral,
    NullLiteral,
    StringLiteral(String),
    Integer(String),
    MinusSign,
    PlusSign,
    Exponent,
    Dot,
    LeftCurly,
    LeftBracket,
    RightCurly,
    RightBracket,
    Comma,
    Colon,
    Error(ErrorInfo),
}

pub type UTF8Source = mpsc::Receiver<String>;
pub type TokenPair = (MaybeToken, TokenInfo);
pub type TokenSink = mpsc::Sender<TokenPair>;

fn is_carriage_return(c: char) -> bool {
    c == '\r'
}

fn is_linefeed(c: char) -> bool {
    c == '\n'
}

fn is_double_quote(c: char) -> bool {
    c == '"'
}

fn is_backslash(c: char) -> bool {
    c == '\\'
}

fn is_digit(c: char) -> bool {
    c == '0'
        || c == '1'
        || c == '2'
        || c == '3'
        || c == '4'
        || c == '5'
        || c == '6'
        || c == '7'
        || c == '8'
        || c == '9'
}

pub fn lex(utf8_source: UTF8Source, lex_output_sink: TokenSink) -> () {
    thread::spawn(move || {
        // all this state is not very rust-esque but that's to figure out later.
        // i can't just look ahead at the rest of the string and capture as much
        // info as i want, because it might not exist yet in the stream. but a
        // lot of this begs to be moved to the actual matching phase
        let mut expected_to_match: &str = "";
        let mut current_token: Option<MaybeToken> = None;
        let mut current_token_info = TokenInfo {
            line_number: 1,
            char_position: 1,
            start: 0,
            length: 0,
        };
        let mut temp_string = String::from("");
        let mut expect_possible_linefeed = false;
        let mut expect_escaped_char = false;
        let mut expected_hex_digits = 0;
        for source_string in utf8_source {
            for source_char in source_string.chars() {
                if let Some(MaybeToken::StringLiteral(s)) = &mut current_token {
                    if expected_hex_digits > 0 {
                        // must take care here. rfc 8259 says they can be upper or lowercase
                        // which implies that mixing is fine
                        match source_char {
                            'a' | 'A' | 'b' | 'B' | 'c' | 'C' | 'd' | 'D' | 'e' | 'E' | 'f'
                            | 'F' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                expected_hex_digits -= 1;
                                s.push(source_char);
                                current_token_info.length += 1;
                                current_token_info.char_position += 1;
                            }
                            _ => {
                                lex_output_sink
                                    .send((MaybeToken::Error(ErrorInfo {
                                        message: format!("The character '{}' is not valid in this context. Only hexadecimal digits (0-9, a-f, A-F) may follow \\u", source_char),
                                        fragment: None
                                    }), current_token_info))
                                .unwrap();
                                return;
                            }
                        }
                        continue;
                    }
                    if expect_escaped_char {
                        expect_escaped_char = false;
                        match source_char {
                            '"' | 'b' | '/' | '\\' | 'f' | 'n' | 'r' | 't' => {
                                s.push(source_char);
                                current_token_info.length += 1;
                                current_token_info.char_position += 1;
                            }
                            'u' => {
                                s.push(source_char);
                                current_token_info.length += 1;
                                current_token_info.char_position += 1;
                                expected_hex_digits = 4;
                            }
                            _ => {
                                lex_output_sink
                                .send((MaybeToken::Error(ErrorInfo {
                                    message: format!("Unsupported escaped character '{}'. Only 'b', 'f', 'n', 'r', 't', 'u<4-digit hex>', '\', or '\"' are allowed", source_char),
                                    fragment: None
                                }), current_token_info))
                            .unwrap();
                                return;
                            }
                        }
                        continue;
                    }
                    if is_double_quote(source_char) {
                        s.push(source_char);
                        current_token_info.length += 1;
                        current_token_info.char_position += 1;
                        // we're finishing the string. send it
                        lex_output_sink
                            .send((current_token.take().unwrap(), current_token_info.clone()))
                            .unwrap();
                        // advance for the next token
                        current_token_info.start += current_token_info.length;
                    } else if is_linefeed(source_char) || is_carriage_return(source_char) {
                        lex_output_sink
                            .send((
                                MaybeToken::Error(ErrorInfo {
                                    message: String::from("Multi-line strings are not allowed"),
                                    fragment: None,
                                }),
                                current_token_info,
                            ))
                            .unwrap();
                        return;
                    } else if is_backslash(source_char) {
                        expect_escaped_char = true;
                        s.push(source_char);
                        current_token_info.length += 1;
                        current_token_info.char_position += 1;
                    } else {
                        s.push(source_char);
                        current_token_info.length += 1;
                        current_token_info.char_position += 1;
                    }
                    continue;
                } else if let Some(MaybeToken::Integer(n)) = &mut current_token {
                    if is_digit(source_char) {
                        n.push(source_char);
                        current_token_info.length += 1;
                        current_token_info.char_position += 1;
                        continue;
                    } else {
                        lex_output_sink
                            .send((current_token.take().unwrap(), current_token_info.clone()))
                            .unwrap();
                        // start a new token
                        current_token_info.start += current_token_info.length;
                        current_token_info.length = 0;
                    }
                }
                if expect_possible_linefeed {
                    if is_linefeed(source_char) {
                        // we're in a CRLF, just skip the char
                        current_token_info.start += 1;
                        if current_token_info.char_position != 1 {
                            panic!("this shouldn't happen");
                        }
                        continue;
                    } // else, ok for some reason there's only a CR. json doesn't seem to forbid that so..
                    expect_possible_linefeed = false;
                }
                if expected_to_match.len() > 0 {
                    if expected_to_match.chars().next().unwrap() == source_char {
                        if expected_to_match.len() == 1 {
                            expected_to_match = "";
                            lex_output_sink
                                .send((current_token.take().unwrap(), current_token_info.clone()))
                                .unwrap();
                            // start over
                            current_token_info.start += current_token_info.length;
                        } else {
                            expected_to_match = &expected_to_match[1..];
                        }
                    } else {
                        // not a match. looks like an error
                        let mut take_string = String::from("");
                        std::mem::swap(&mut temp_string, &mut take_string);
                        lex_output_sink
                            .send((
                                MaybeToken::Error(ErrorInfo {
                                    message: String::from("Unrecognized token"),
                                    fragment: Some(take_string),
                                }),
                                current_token_info,
                            ))
                            .unwrap();
                        return;
                    }
                    current_token_info.char_position += 1;
                    continue;
                } else if let None = &current_token {
                    let mut keep_going = true;
                    if source_char == 'f' {
                        expected_to_match = "alse";
                        current_token = Some(MaybeToken::FalseLiteral);
                        current_token_info.length = "false".len();
                    } else if source_char == 't' {
                        expected_to_match = "rue";
                        current_token = Some(MaybeToken::TrueLiteral);
                        current_token_info.length = "true".len();
                    } else if source_char == 'n' {
                        expected_to_match = "ull";
                        current_token = Some(MaybeToken::NullLiteral);
                        current_token_info.length = "null".len();
                    } else {
                        keep_going = false;
                    }
                    if keep_going {
                        current_token_info.char_position += 1;
                        continue;
                    }
                }

                let matched_simple_token: Option<MaybeToken>;
                match source_char {
                    '"' => {
                        if current_token.is_some() {
                            // send the old token first
                            lex_output_sink
                                .send((current_token.take().unwrap(), current_token_info.clone()))
                                .unwrap();
                            current_token_info.start += current_token_info.length;
                        }
                        // start a new string token
                        current_token = Some(MaybeToken::StringLiteral(String::from("\"")));
                        current_token_info.char_position += 1;
                        current_token_info.length = 1;
                        continue;
                    }
                    '\r' => {
                        expect_possible_linefeed = true;
                        current_token_info.line_number += 1;
                        current_token_info.char_position = 1;
                        current_token_info.start += 1;
                        continue;
                    }
                    '\n' => {
                        current_token_info.line_number += 1;
                        current_token_info.char_position = 1;
                        current_token_info.start += 1;
                        continue;
                    }
                    ' ' | '\t' => {
                        current_token_info.char_position += 1;
                        current_token_info.start += 1;
                        continue;
                    }
                    '-' => matched_simple_token = Some(MaybeToken::MinusSign),
                    '+' => matched_simple_token = Some(MaybeToken::PlusSign),
                    '{' => matched_simple_token = Some(MaybeToken::LeftCurly),
                    '}' => matched_simple_token = Some(MaybeToken::RightCurly),
                    '[' => matched_simple_token = Some(MaybeToken::LeftBracket),
                    ']' => matched_simple_token = Some(MaybeToken::RightBracket),
                    ',' => matched_simple_token = Some(MaybeToken::Comma),
                    'e' | 'E' => matched_simple_token = Some(MaybeToken::Exponent),
                    '.' => matched_simple_token = Some(MaybeToken::Dot),
                    ':' => matched_simple_token = Some(MaybeToken::Colon),
                    _ => matched_simple_token = None,
                }

                if let Some(mst) = matched_simple_token {
                    current_token = None; // make sure this is unset. we don't need it
                    current_token_info.length = 1;
                    current_token_info.char_position += 1;
                    lex_output_sink
                        .send((mst, current_token_info.clone()))
                        .unwrap();
                    current_token_info.start += 1; // advance and
                    current_token_info.length = 0; // reset
                    continue;
                }

                if is_digit(source_char) {
                    // looks like a number...
                    current_token_info.char_position += 1;
                    current_token_info.length += 1;
                    let mut tmp_str = String::new();
                    tmp_str.push(source_char);
                    current_token = Some(MaybeToken::Integer(tmp_str));
                } else {
                    lex_output_sink
                        .send((
                            MaybeToken::Error(ErrorInfo {
                                message: format!(
                                    "Encountered an unexpected character '{}'",
                                    source_char
                                ),
                                fragment: None,
                            }),
                            current_token_info,
                        ))
                        .unwrap();
                    return;
                }
            }
        }

        // the stream might have ended while we were constructing certain tokens.
        // if it's a quoted string, that's an error
        // but on the off-chance it's a digit, ok, send it
        if let Some(MaybeToken::Integer(_)) = &current_token {
            // the number is finished. we're on a new token
            lex_output_sink
                .send((current_token.take().unwrap(), current_token_info.clone()))
                .unwrap();
            return;
        }

        if current_token_info.line_number == 1 && current_token_info.char_position == 1 {
            // todo: fix the error-handling. i don't like this...
            lex_output_sink
                .send((
                    MaybeToken::Error(ErrorInfo {
                        message: String::from("Source cannot be empty"),
                        fragment: None,
                    }),
                    current_token_info,
                ))
                .unwrap();
            return;
        }
    });
}
