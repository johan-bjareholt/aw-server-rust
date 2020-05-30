use regex::Regex;
use serde_json::value::Value;

use aw_models::Event;

pub fn filter_keyvals(events: Vec<Event>, key: &str, vals: &[Value]) -> Vec<Event> {
    let mut filtered_events = Vec::new();
    for event in events {
        match event.data.get(key) {
            Some(v) => {
                for val in vals {
                    if val == v {
                        filtered_events.push(event.clone());
                        break;
                    }
                }
            }
            None => break,
        }
    }
    filtered_events
}

pub fn filter_keyvals_regex(events: Vec<Event>, key: &str, regex: &Regex) -> Vec<Event> {
    let mut filtered_events = Vec::new();

    for event in events {
        match event.data.get(key) {
            Some(v) => {
                if regex.is_match(v.as_str().unwrap()) {
                    filtered_events.push(event.clone());
                }
            }
            None => (),
        }
    }
    filtered_events
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::DateTime;
    use chrono::Duration;
    use regex::RegexBuilder;
    use serde_json::json;

    use aw_models::Event;

    use super::{filter_keyvals, filter_keyvals_regex};

    #[test]
    fn test_filter_keyvals() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(1),
            data: json_map! {"test": json!(1)},
        };
        let mut e2 = e1.clone();
        e2.data = json_map! {"test": json!(1), "test2": json!(1)};
        let mut e3 = e1.clone();
        e3.data = json_map! {"test2": json!(2)};
        let res = filter_keyvals(vec![e1.clone(), e2.clone(), e3], "test", &vec![json!(1)]);
        assert_eq!(vec![e1, e2], res);
    }

    #[test]
    fn test_filter_keyvals_regex() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(1),
            data: json_map! {"key1": json!("value1")},
        };
        let mut e2 = e1.clone();
        e2.data = json_map! {"key1": json!("value2")};
        let mut e3 = e1.clone();
        e3.data = json_map! {"key2": json!("value3")};

        let events = vec![e1.clone(), e2.clone(), e3.clone()];

        let regex_value = RegexBuilder::new("value").build().unwrap();
        let regex_value1 = RegexBuilder::new("value1").build().unwrap();

        let res = filter_keyvals_regex(events.clone(), "key1", &regex_value);
        assert_eq!(vec![e1.clone(), e2.clone()], res);
        let res = filter_keyvals_regex(events.clone(), "key1", &regex_value1);
        assert_eq!(vec![e1.clone()], res);
        let res = filter_keyvals_regex(events.clone(), "key2", &regex_value);
        assert_eq!(vec![e3.clone()], res);
        let res = filter_keyvals_regex(events.clone(), "key2", &regex_value1);
        assert_eq!(0, res.len());
        let res = filter_keyvals_regex(events.clone(), "key3", &regex_value);
        assert_eq!(0, res.len());
    }
}
