use crate::lexer;
use std::{sync::mpsc, thread};

pub type TagInfo = lexer::TokenInfo;
pub type ErrorInfo = lexer::ErrorInfo;

#[derive(PartialEq, std::fmt::Debug)]
pub enum Tag {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    ObjectKey(String),
    StringLiteral(String),
    Number(String),
    TrueLiteral,
    FalseLiteral,
    NullLiteral,
    Error(ErrorInfo, TagInfo),
}

pub type TokenPair = lexer::TokenPair;
pub type TokenSource = mpsc::Receiver<TokenPair>;
pub type TokenIter<'a> = mpsc::Iter<'a, TokenPair>;
pub type TagSource = mpsc::Sender<Tag>;
// convenience types for outside users
pub type TagSink = mpsc::Receiver<Tag>;
pub type TagVec = Vec<Tag>;

fn strip_string_quotes(s: &str) -> &str {
    let len = s.len();
    if len == 2 {
        ""
    } else if len > 2 {
        &s[1..len - 1]
    } else {
        panic!("Something went dreadfully wrong in the lexer");
    }
}

fn require_object_kv_pair(
    key: (&String, &lexer::TokenInfo),
    rest: &mut TokenIter,
    sink: &TagSource,
) -> (bool,Option<TokenPair>) {
    let stripped_name = strip_string_quotes(key.0).to_string();
    sink.send(Tag::ObjectKey(stripped_name.clone())).unwrap();
    let next_token = require_next_token(rest, sink, &key.1);
    if next_token.is_ok() {
        let next_token = next_token.unwrap();
        if let (lexer::MaybeToken::Colon, info) = &next_token {
            let hopefully_value_token = require_next_token(rest, sink, info);
            if hopefully_value_token.is_ok() {
                let result = try_value(&hopefully_value_token.unwrap(), rest, sink);
                if result.is_err() {
                    sink.send(Tag::Error(
                        ErrorInfo {
                            message: format!("A value is required after `\"{}\": `", stripped_name),
                            fragment: None,
                        },
                        next_token.1,
                    ))
                    .unwrap();
                    return (false,None);
                }
                let result = result.unwrap();
                if !result.0 {
                    sink.send(Tag::Error(
                        ErrorInfo {
                            message: format!("A value is required after `\"{}\": `", stripped_name),
                            fragment: None,
                        },
                        next_token.1,
                    ))
                    .unwrap();
                }
                return result;
            }
        } else {
            sink.send(Tag::Error(
                ErrorInfo {
                    message: format!("Expected ':' after the key '{}'", stripped_name),
                    fragment: None,
                },
                next_token.1,
            ))
            .unwrap();
        }
    }
    (false,None)
}

fn try_object(start_token: &TokenPair, rest: &mut TokenIter, sink: &TagSource) -> Result<bool, ()> {
    if let lexer::MaybeToken::LeftCurly = &start_token.0 {
        if sink.send(Tag::BeginObject).is_err() {
            return Err(());
        }

        if let Ok(mut next_token) = require_next_token(rest, sink, &start_token.1) {
            let mut require_comma_or_curly = false;
            let mut require_kv_pair = false;
            loop {
                match &next_token {
                    // possible kv pair
                    (lexer::MaybeToken::StringLiteral(key_name), key_name_info) => {
                        let res = require_object_kv_pair((&key_name, &key_name_info), rest, sink);
                        if !res.0 {
                            return Err(());
                        }
                        require_comma_or_curly = true;
                        require_kv_pair = false;
                        if res.1.is_some() {
                            next_token = res.1.unwrap();
                            // we have a lookahead left over, so bypass the next token code
                            // after match
                            continue;
                        }
                    }
                    (lexer::MaybeToken::Comma, info) => {
                        if require_comma_or_curly {
                            require_kv_pair = true;
                            require_comma_or_curly = false;
                        } else {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from(
                                        "Syntax error. Encountered unexpected ',' in object",
                                    ),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                    }
                    (lexer::MaybeToken::RightCurly, info) => {
                        if require_kv_pair {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from("Expected another key-value pair after comma, but found a '}'"),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                        sink.send(Tag::EndObject).unwrap();
                        return Ok(true);
                    }
                    (_, info) => {
                        if require_kv_pair {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from("Expected another key-value pair"),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                        if require_comma_or_curly {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from("Expected comma or closing curly brace"),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                        // the general case. most likely the object looks like { false, ...}
                        // because someone forgot to quote the key name
                        sink.send(Tag::Error(
                            ErrorInfo {
                                message: String::from(
                                    "Expected key-value pair or closing curly brace",
                                ),
                                fragment: None,
                            },
                            info.clone(),
                        ))
                        .unwrap();
                        return Err(());
                    }
                }
                // load the next token (if there is one)
                let temp = require_next_token(rest, sink, &start_token.1);
                if temp.is_ok() {
                    next_token = temp.unwrap();
                } else {
                    return Err(());
                }
            }
        } else {
            return Err(());
        }
    }
    Ok(false)
}

fn try_array(start_token: &TokenPair, rest: &mut TokenIter, sink: &TagSource) -> Result<bool, ()> {
    if let lexer::MaybeToken::LeftBracket = &start_token.0 {
        if sink.send(Tag::BeginArray).is_err() {
            return Err(());
        }

        if let Ok(mut next_token) = require_next_token(rest, sink, &start_token.1) {
            let mut require_comma_or_bracket = false;
            let mut require_value_tok = false;
            loop {
                match &next_token {
                    (lexer::MaybeToken::Comma, info) => {
                        if require_comma_or_bracket {
                            require_value_tok = true;
                            require_comma_or_bracket = false;
                        } else {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from(
                                        "Syntax error. Encountered unexpected ',' in array",
                                    ),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                    }
                    (lexer::MaybeToken::RightBracket, info) => {
                        if require_value_tok {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from("Expected another value after comma, but found a ']'"),
                                    fragment: None,
                                },
                                info.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                        sink.send(Tag::EndArray).unwrap();
                        return Ok(true);
                    }
                    any_pair => {
                        if require_comma_or_bracket {
                            sink.send(Tag::Error(
                                ErrorInfo {
                                    message: String::from("Expected comma or closing bracket"),
                                    fragment: None,
                                },
                                any_pair.1.clone(),
                            ))
                            .unwrap();
                            return Err(());
                        }
                        {
                            let result = try_value(any_pair, rest, sink);
                            if result.is_err() {
                                sink.send(Tag::Error(
                                    ErrorInfo {
                                        message: String::from("Expected value or closing bracket"),
                                        fragment: None,
                                    },
                                    any_pair.1.clone(),
                                ))
                                .unwrap();
                                return Err(());
                            } 
                            let result = result.unwrap();
                            if !result.0 {
                                sink.send(Tag::Error(
                                    ErrorInfo {
                                        message: String::from("Expected value or closing bracket"),
                                        fragment: None,
                                    },
                                    any_pair.1.clone(),
                                ))
                                .unwrap();
                                return Err(());
                            }
                            if let Some(lookahead) = result.1 {
                                // ok value succeeded, but we have a lookahead token
                                next_token = lookahead;
                                require_comma_or_bracket = true;
                                require_value_tok = false;
                                continue;
                            } else {
                                require_comma_or_bracket = true;
                                require_value_tok = false;
                            }
                        }
                    }
                }
                // load the next token (if there is one)
                let temp = require_next_token(rest, sink, &start_token.1);
                if temp.is_ok() {
                    next_token = temp.unwrap();
                } else {
                    return Err(());
                }
            }
        } else {
            return Err(());
        }
    }
    Ok(false)
}

fn try_string(start_token: &TokenPair, sink: &TagSource) -> Result<bool, ()> {
    if let lexer::MaybeToken::StringLiteral(s) = &start_token.0 {
        // unlike in the lexer where we need to preserve char positions for all tokens
        // we strip the enclosing quotes off strings here because they'd otherwise
        // be tedious for users of the parser
        if sink
            .send(Tag::StringLiteral(strip_string_quotes(s).to_string()))
            .is_err()
        {
            return Err(());
        } else {
            return Ok(true);
        }
    }
    Ok(false)
}

fn require_integer(next_token: &lexer::MaybeToken) -> Result<String, ()> {
    if let lexer::MaybeToken::Integer(n) = next_token {
        Ok(n.clone())
    } else {
        Err(())
    }
}

// possibly advances the iter by 1 token pair. if the next token is an Error, it sends
// it to the sink and returns an Err result for propagating an abort.
// note that if the next token doesn't exist because the stream ended, this is
// not treated as an error because in some cases it's ok and just means the parse
// has finished
fn try_next_token(source: &mut TokenIter, sink: &TagSource) -> Result<Option<TokenPair>, ()> {
    match source.next() {
        Some((token, info)) => {
            if let lexer::MaybeToken::Error(err_info) = token {
                sink.send(Tag::Error(err_info, info)).unwrap();
                Err(())
            } else {
                Ok(Some((token, info)))
            }
        }
        None => Ok(None),
    }
}

// the last token info is only needed to output a helpful error message in case the stream ends
// unexpectedly
fn require_next_token(
    source: &mut TokenIter,
    sink: &TagSource,
    last_token_info: &lexer::TokenInfo,
) -> Result<TokenPair, ()> {
    let result = try_next_token(source, sink);
    if result.is_err() {
        Err(())
    } else {
        let pair = result?;
        if pair.is_none() {
            let dupe = lexer::TokenInfo {
                char_position: last_token_info.char_position + 1,
                start: last_token_info.start + last_token_info.length,
                length: 0,
                line_number: last_token_info.line_number,
            };
            sink.send(Tag::Error(
                ErrorInfo {
                    message: String::from("Encountered end of stream, but more tokens expected"),
                    fragment: None,
                },
                dupe,
            ))
            .unwrap();
            Err(())
        } else {
            Ok(pair.unwrap())
        }
    }
}

fn try_number(start_token: &TokenPair, rest: &mut TokenIter, sink: &TagSource) -> Result<(bool,Option<TokenPair>), ()> {
    let mut recomposed_float = String::from("");
    let mut new_tok_info = start_token.1.clone();
    new_tok_info.length = 0; // reset, we're going to manually count the length

    let error_sender = |msg: &str, tag_info: &TagInfo| {
        sink.send(Tag::Error(
            ErrorInfo {
                message: String::from(msg),
                fragment: None,
            },
            tag_info.clone(),
        ))
        .unwrap();
    };

    let send_num_and_return = |tok,leftover_tok| -> Result<(bool,Option<TokenPair>), ()> {
        sink.send(tok).unwrap();
        Ok((true,leftover_tok))
    };

    // in case we need to give back the lookahead, which we cannot really return to a
    // mpsc::Receiver
    let mut leftover_token : Option<TokenPair> = None;

    if let (lexer::MaybeToken::MinusSign, info) = start_token {
        let next_token = require_next_token(rest, sink, info);
        if next_token.is_err() {
            return Err(());
        }
        recomposed_float.push('-');
        new_tok_info.length += info.length;
        new_tok_info.char_position = info.char_position;
        let (tok, info) = next_token?;
        if let Ok(intval) = require_integer(&tok) {
            recomposed_float.push_str(&intval);
            new_tok_info.length += info.length;
            new_tok_info.char_position = info.char_position;
        } else {
            error_sender("Syntax error. Expected digits after '-'", &info);
            return Err(());
        }
    } else if let (lexer::MaybeToken::Integer(n), info) = start_token {
        recomposed_float.push_str(n);
        new_tok_info.length += info.length;
        new_tok_info.char_position = info.char_position;
    } else {
        return Ok((false,None));
    }
    {
        // check for optional fraction, optional exponent
        let mut next_token = try_next_token(rest, sink);
        if next_token.is_err() {
            return Err(());
        }
        let opt_pair = next_token?;
        if !opt_pair.is_none() {
            let mut pair = opt_pair.unwrap();
            if let (lexer::MaybeToken::Dot, info) = pair {
                recomposed_float.push('.');
                new_tok_info.length += info.length;
                new_tok_info.char_position = info.char_position;
                let hopefully_int = require_next_token(rest, sink, &info);
                if hopefully_int.is_err() {
                    return Err(());
                }
                let int_pair = hopefully_int?;
                if let Ok(intval) = require_integer(&int_pair.0) {
                    recomposed_float.push_str(&intval);
                    let int_info = int_pair.1;
                    new_tok_info.length += int_info.length;
                    new_tok_info.char_position = int_info.char_position;
                } else {
                    error_sender("Syntax error. Expected fractional digits after '.'", &info);
                    return Err(());
                }
                // great, we found the fractional part. now set everything back up
                // for the exponent code
                next_token = try_next_token(rest, sink);
                if next_token.is_err() {
                    return Err(());
                }
                let opt_pair = next_token?;
                if !opt_pair.is_none() {
                    pair = opt_pair.unwrap();
                } else {
                    // we're done
                    return send_num_and_return(Tag::Number(recomposed_float), None);
                }
            }
            if let (lexer::MaybeToken::Exponent, info) = pair {
                recomposed_float.push('E');
                new_tok_info.length += info.length;
                new_tok_info.char_position = info.char_position;
                // check for (+|-)<int>
                next_token = try_next_token(rest, sink);
                if next_token.is_err() {
                    return Err(());
                }
                let opt_pair = next_token?;
                if !opt_pair.is_none() {
                    // check for + or - followed by an int or just an int
                    let (mut tok, mut info) = opt_pair.unwrap();
                    let mut was_plus_or_minus = false;
                    if tok == lexer::MaybeToken::PlusSign {
                        recomposed_float.push('+');
                        was_plus_or_minus = true;
                    } else if tok == lexer::MaybeToken::MinusSign {
                        recomposed_float.push('-');
                        was_plus_or_minus = true;
                    }
                    if was_plus_or_minus {
                        new_tok_info.length += info.length;
                        new_tok_info.char_position = info.char_position;
                        next_token = try_next_token(rest, sink);
                        if next_token.is_err() {
                            return Err(());
                        }
                        let opt_pair = next_token?;
                        if !opt_pair.is_none() {
                            // ok rust doesn't allow this according to reddit...
                            // (tok,info) = opt_pair.unwrap();
                            let temp = opt_pair.unwrap();
                            tok = temp.0;
                            info = temp.1;
                        } else {
                            error_sender(
                                "Syntax error. Expected digits after exponent character",
                                &info,
                            );
                            return Err(());
                        }
                    }
                    // the next token MUST be a digit
                    if let Ok(intval) = require_integer(&tok) {
                        recomposed_float.push_str(&intval);
                        new_tok_info.length += info.length;
                        new_tok_info.char_position = info.char_position;
                    } else {
                        error_sender("Syntax error. Expected digits after '-'", &info);
                        return Err(());
                    }
                } else {
                    error_sender(
                        "Syntax error. Expected digits after exponent character",
                        &info,
                    );
                    return Err(());
                }
            } else {
                // oops, we took a token that didn't belong to us
                leftover_token = Some(pair);
            }
        } // else it's just an integer and we're done
    }

    send_num_and_return(Tag::Number(recomposed_float), leftover_token)
}

fn try_literal(start_token: &TokenPair, sink: &TagSource) -> Result<bool, ()> {
    let matching_literal = match start_token.0 {
        lexer::MaybeToken::FalseLiteral => Tag::FalseLiteral,
        lexer::MaybeToken::TrueLiteral => Tag::TrueLiteral,
        lexer::MaybeToken::NullLiteral => Tag::NullLiteral,
        _ => return Ok(false),
    };
    // i vaguely recall there's some way to convert one result type to another, but
    // i cannot find it now, unfortunately
    if sink.send(matching_literal).is_err() {
        Err(())
    } else {
        Ok(true)
    }
}

fn try_value(start_token: &TokenPair, rest: &mut TokenIter, sink: &TagSource) -> Result<(bool,Option<TokenPair>), ()> {
    // not sure how to elegantly do this. Result has a billion functions
    // but none of them really match what I want:
    // if Ok(true) or Err short-circuit, else call the next try fn
    let worked = try_object(start_token, rest, sink);
    if worked.is_err() {
        return Err(());
    } else if worked.unwrap() {
        return Ok((true,None));
    }
    let worked = try_array(start_token, rest, sink);
    if worked.is_err() {
        return Err(());
    } else if worked.unwrap() {
        return Ok((true,None));
    }
    let worked = try_string(start_token, sink);
    if worked.is_err() {
        return Err(());
    } else if worked.unwrap() {
        return Ok((true,None));
    }
    let worked = try_literal(start_token, sink);
    if worked.is_err() {
        return Err(());
    } else if worked.unwrap() {
        return Ok((true,None));
    }
    let worked = try_number(start_token, rest, sink);
    if worked.is_err() {
        return Err(());
    } else if let (true,lookahead) = worked.unwrap() {
        return Ok((true,lookahead));
    }
    Err(())
}

pub fn parse(token_source: TokenSource, tag_sink: TagSource) -> () {
    thread::spawn(move || {
        let mut token_iter = token_source.iter();
        match token_iter.next() {
            Some(token_pair) => {
                if let (lexer::MaybeToken::Error(err_info), info) = token_pair {
                    tag_sink.send(Tag::Error(err_info, info)).unwrap();
                    return;
                }
                let result = try_value(&token_pair, &mut token_iter, &tag_sink);
                if result.is_err() {
                    // we _should have_ already handled it
                }
                return;
            }
            None => {}
        }
    });
}
