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
        return self.data.get(key);
    }
}

impl<'buffer_lifetime> From<&'buffer_lifetime str> for QueryString<'buffer_lifetime> {
    fn from(s: &'buffer_lifetime str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Multiple(vec) => vec.push(val),
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                })
                .or_insert(Value::Single(val));
        }

        return QueryString { data };
    }
}
