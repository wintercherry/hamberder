pub mod lexer;
pub mod parser;

pub fn parse(utf8_source: lexer::UTF8Source) -> parser::TagSink {
    let (token_tx, token_rx) = std::sync::mpsc::channel();
    let (tag_tx, tag_rx) = std::sync::mpsc::channel();
    lexer::lex(utf8_source, token_tx);
    parser::parse(token_rx, tag_tx);
    tag_rx
}

use std::error::Error;

// note: while lexing and parsing always happen in the background, parse_file(...)
// blocks until the entire file has been passed to the lexer so that it can return
// a success/failure flag for reading the contents of the file
pub fn parse_file(file_path: &str) -> Result<parser::TagSink,Box<dyn Error>> {
    use std::{fs::File, io::BufReader, io::Read};
    let f = File::open(&file_path)?;
    let (string_tx, string_rx) = std::sync::mpsc::channel();
    let mut reader = BufReader::new(f);
    let tag_sink = parse(string_rx);
    loop {
        const SOME_ARBITRARY_CAPACITY: usize = 8 * 1024;
        let mut line: Vec<u8> = vec![0; SOME_ARBITRARY_CAPACITY];
        let len = reader.read(&mut line)?;
        if len > 0 {
            string_tx.send(String::from_utf8(line)?)?;
        } else {
            break;
        }
    }
    Ok(tag_sink)
}
