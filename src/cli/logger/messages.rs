use serde_json::Value;
use std::fs;

fn load() -> Value {
    let data = fs::read_to_string("assets/messages.json").unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn get(key: &str, args: Option<Vec<String>>) -> String {
    let messages = load();
    let template = messages[key].as_str().unwrap();

    if let Some(args) = args {
        let mut formatted_message = template.to_string();

        for (_, arg) in args.iter().enumerate() {
            formatted_message = formatted_message.replace(&format!("{{}}"), arg);
        }
        formatted_message
    } else {
        template.to_string()
    }
}
