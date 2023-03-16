use std::collections::HashMap;

#[derive(Debug)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    match chars.peek() {
        Some('"') => parse_string(chars),
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('t') | Some('f') => parse_boolean(chars),
        Some('n') => parse_null(chars),
        Some('-') | Some('0'..='9') => parse_number(chars),
        Some(_) => None,
        None => None,
    }
}

fn parse_object(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    let mut object = HashMap::new();
    chars.next();
    loop {
        match chars.peek() {
            Some('}') => {
                chars.next();
                return Some(JsonValue::Object(object));
            }
            Some(',') => {
                chars.next();
            }
            Some(_) => {
                let key = match parse_string(chars) {
                    Some(JsonValue::String(s)) => s,
                    _ => return None,
                };
                if chars.next() != Some(':') {
                    return None;
                }
                let value = match parse_value(chars) {
                    Some(value) => value,
                    _ => return None,
                };
                object.insert(key, value);
            }
            None => return None,
        }
    }
}

fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    chars.next();
    let mut s = String::new();
    loop {
        match chars.next()? {
            '"' => {
                return Some(JsonValue::String(s));
            }
            '\\' => {
                match chars.next()? {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\x08'),
                    'f' => s.push('\x0C'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        // Parse the next four characters as a hex code
                        let hex_code: String = chars.take(4).collect();
                        let code_point = u32::from_str_radix(&hex_code, 16).ok()?;
                        match std::char::from_u32(code_point) {
                            Some(c) => s.push(c),
                            None => return None,
                        }
                    }
                    _ => return None,
                }
            }
            c => s.push(c),
        }
    }
}

fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    let mut array = Vec::new();
    chars.next();
    loop {
        match chars.peek() {
            Some(']') => {
                chars.next();
                return Some(JsonValue::Array(array));
            }
            Some(',') => {
                chars.next();
            }
            Some(_) => match parse_value(chars) {
                Some(value) => array.push(value),
                _ => return None,
            },
            None => return None,
        }
    }
}

fn parse_boolean(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    let mut bool_str = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphabetic() {
            bool_str.push(chars.next()?);
        } else {
            break;
        }
    }
    match bool_str.as_str() {
        "true" => Some(JsonValue::Boolean(true)),
        "false" => Some(JsonValue::Boolean(false)),
        _ => None,
    }
}

fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    let mut null_str = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphabetic() {
            null_str.push(chars.next()?);
        } else {
            break;
        }
    }
    if null_str == "null" {
        Some(JsonValue::Null)
    } else {
        None
    }
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<JsonValue> {
    let mut number_str = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' || ch == 'e' || ch == 'E' {
            number_str.push(chars.next()?);
        } else {
            break;
        }
    }
    match number_str.parse() {
        Ok(number) => Some(JsonValue::Number(number)),
        Err(_) => None,
    }
}
fn main() -> Result<(), ()> {
    let json_str = r#"{
        "name": 1
    }"#;
    let json_value = parse_json(json_str).unwrap();
    println!("{json_value:#?}");

    Ok(())
}

pub fn parse_json(json_str: &str) -> Result<JsonValue, String> {
    let mut chars = json_str.chars().peekable();
    match parse_value(&mut chars) {
        Some(value) => Ok(value),
        None => Err("Invalid JSON input".to_string()),
    }
}
