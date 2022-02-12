// jkcoxson
// Parsing a plist shouldn't be this hard smh

use std::{collections::HashMap, fmt::Debug};

use regex::Regex;

enum PlistValues {
    String(String),
    Integer(i64),
    Boolean(bool),
    Data(Vec<u8>),
    Array(Vec<PlistValues>),
    Dictionary(HashMap<String, PlistValues>),
}

pub struct Plist {
    hashmap: HashMap<String, PlistValues>,
}

impl Plist {
    fn from_string(string: String) -> Plist {
        let mut hashmap = HashMap::new();
        let bracket_regex = Regex::new(r"<(.*?)>").unwrap();
        for i in 0..bracket_regex.captures_iter(&string).count() {
            let mut capture = bracket_regex.captures_iter(&string);
            let key = capture.nth(i).unwrap().get(1).unwrap().as_str();
            let key_location = capture.nth(i).unwrap().get(1).unwrap().start();
            println!("Key: {}", key);
            let end_key = capture.nth(i + 1).unwrap().get(1).unwrap().start();
            let end_key_location = capture.nth(i).unwrap().get(1).unwrap().start();
            println!("End key: {}", end_key);

            let value = capture.nth(i + 2).unwrap().get(1).unwrap().as_str();
            let value_location = capture.nth(i + 2).unwrap().get(1).unwrap().start();
            println!("Value: {}", value);
            let end_value = capture.nth(i + 3).unwrap().get(1).unwrap().start();
            let end_value_location = capture.nth(i + 3).unwrap().get(1).unwrap().start();
            println!("End value: {}", end_value);

            match value {
                "string" => {
                    let string_value =
                        string[value_location + 7..end_value_location - 1].to_string();
                    hashmap.insert(key, PlistValues::String(string_value));
                }
                "true/" => {
                    hashmap.insert(key, PlistValues::Boolean(true));
                }
                "false/" => {
                    hashmap.insert(key.clone(), PlistValues::Boolean(false));
                }
                "integer" => {
                    let integer_value =
                        string[value_location + 9..end_value_location - 1].to_string();
                    hashmap.insert(
                        key.clone(),
                        PlistValues::Integer(integer_value.parse::<i64>().unwrap()),
                    );
                }
                "dict" => {
                    let dict_value = Plist::from_string(
                        string[value_location + 5..end_value_location - 1].to_string(),
                    );
                }
                "array" => {
                    //todo!();
                }
                "data" => {
                    let data_value = string[value_location + 5..end_value_location - 1].to_string();
                    hashmap.insert(key, PlistValues::Data(data_value.into()));
                }
                _ => {
                    panic!("Unsupported value type: {}", value);
                }
            }
        }

        todo!()
    }
}

impl Into<Plist> for String {
    fn into(self) -> Plist {
        Plist::from_string(self)
    }
}

impl Into<Plist> for &str {
    fn into(self) -> Plist {
        self.to_string().into()
    }
}

impl Debug for Plist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plist {{ hashmap: {:?} }}", self.hashmap)
    }
}

impl Debug for PlistValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlistValues::String(string) => write!(f, "String({})", string),
            PlistValues::Integer(integer) => write!(f, "Integer({})", integer),
            PlistValues::Boolean(boolean) => write!(f, "Boolean({})", boolean),
            PlistValues::Data(data) => write!(f, "Data({:?})", data),
            PlistValues::Array(array) => write!(f, "Array({:?})", array),
            PlistValues::Dictionary(dictionary) => write!(f, "Dictionary({:?})", dictionary),
        }
    }
}
