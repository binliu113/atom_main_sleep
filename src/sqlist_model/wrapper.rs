use rocket::form::validate::Contains;
use crate::sqlist_model::util::{Column, ValueType};


pub struct QueryWrapper {
    pub query: String,
    query_fmt: Vec<String>,
}

impl QueryWrapper {
    pub fn new() -> Self {
        Self {
            query: String::from(""),
            query_fmt: vec![String::from("WHERE")],
        }
    }

    fn str_for_query(field: &String, wh: &str, value: ValueType) -> Option<String> {
        match value {
            ValueType::INTEGER(val) => {
                match val {
                    None => None,
                    Some(v) => {
                        Some(format!("{}{}{}", field.clone(), wh, v))
                    }
                }
            }
            ValueType::TEXT(val) => {
                match val {
                    None => None,
                    Some(v) => {
                        Some(format!("{}{}'{}'", field.clone(), wh, v))
                    }
                }
            }
            ValueType::BLOB(val) => {
                match val {
                    None => None,
                    Some(v) => {
                        Some(format!("{}{}'{}'", field.clone(), wh, v))
                    }
                }
            }
            ValueType::REAL(val) => {
                match val {
                    None => None,
                    Some(v) => {
                        Some(format!("{}{}{}", field.clone(), wh, v))
                    }
                }
            }
            ValueType::NULL(val) => {
                match val {
                    None => None,
                    Some(v) => {
                        Some(format!("{}{}'{}'", field.clone(), wh, v))
                    }
                }
            }
        }
    }

    fn relation(&mut self, rlt: &str) {
        let len: i32 = (*&self.query_fmt.len().clone() as i32) - &1;
        if &len >= (&0) {
            let item: &str = &self.query_fmt.get(*&len as usize).unwrap();
            if !vec!["AND", "OR", "WHERE"].contains(item) {
                let _ = &self.query_fmt.push(rlt.to_string().clone());
            }
        }
    }

    #[allow(dead_code)]
    pub fn debug(&self) {
        let query = &self.query_fmt.join(" ");
        log::info!("{:?}",query);
    }

    #[allow(dead_code)]
    pub fn or(&mut self) -> &mut Self {
        let _ = &self.relation("OR");
        self
    }

    #[allow(dead_code)]
    pub fn and(&mut self) -> &mut Self {
        let _ = &self.relation("AND");
        self
    }

    #[allow(dead_code)]
    pub fn eq(&mut self, column: &Column, value: ValueType) -> &mut Self {
        match Self::str_for_query(&column.field, "=", value) {
            Some(str) => {
                self.and();
                let _ = &self.query_fmt.push(str);
            }
            None => {}
        };
        self
    }

    #[allow(dead_code)]
    fn ne(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn ge(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn gt(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn le(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn lt(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn is_null(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn is_not_null(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn between(&self) -> String {
        "12".to_string()
    }

    #[allow(dead_code)]
    fn not_between(&self) -> String {
        "12".to_string()
    }
}