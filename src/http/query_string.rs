use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buffer_lifetime> {
    data: HashMap<&'buffer_lifetime str, Value<'buffer_lifetime>>,
}

#[derive(Debug)]
pub enum Value<'buffer_lifetime> {
    Single(&'buffer_lifetime str),
    Multiple(Vec<&'buffer_lifetime str>),
}

impl<'buffer_lifetime> QueryString<'buffer_lifetime> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer_lifetime> From<&'buffer_lifetime str> for QueryString<'buffer_lifetime> {
    fn from(s: &'buffer_lifetime str) -> Self {
        let mut data = HashMap::new();

        for substring in s.split('&') {
            let mut key = substring;
            let mut value = "";
            if let Some(index) = substring.find('=') {
                key = &substring[..index];
                value = &substring[index + 1..];
            }
            data.entry(key).and_modify(|existing: &mut Value| match existing {
                Value::Single(previous_value) => {
                    *existing = Value::Multiple(vec![previous_value, value]);
                },
                Value::Multiple(vector) => vector.push(value),
            }).or_insert(Value::Single(value));
        }
        QueryString { data }
    }
}
