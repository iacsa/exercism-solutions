use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn get_attr(&self, attr: &str) -> Option<&str> {
        self.attrs.get(attr).map(|c| &c[..])
    }
}
