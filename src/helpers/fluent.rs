use std::collections::HashMap;
use std::string::String;

use fluent::MessageContext;

pub struct Fluent<'context> {
    pub contexts: HashMap<String, MessageContext<'context>>,
}

impl<'context> Fluent<'context> {
    pub fn load(
        _theme: String,
        _themes_dir: String
    ) -> Fluent<'context> {
        let contexts: HashMap<String, MessageContext<'context>> = HashMap::new();

        Fluent {
            contexts
        }
    }
}