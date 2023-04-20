use super::{query_types::QueryType, query_classes::QueryClass};

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub query_type: QueryType,
    pub query_class: QueryClass,
}

impl Question {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            query_type: QueryType::A,
            query_class: QueryClass::IN,
        }
    }

    pub fn new(name: String, query_type: QueryType, query_class: QueryClass) -> Self {
        Self {
            name,
            query_type,
            query_class,
        }
    }
}
