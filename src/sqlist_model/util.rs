use std::fmt::{Debug};
use async_trait::async_trait;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use crate::sqlist_model::wrapper::{QueryWrapper};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ValueType {
    #[serde(alias = "integer")]
    /// i32
    INTEGER(Option<i32>),

    /// string
    #[serde(alias = "text")]
    TEXT(Option<String>),

    /// string
    #[serde(alias = "blob")]
    BLOB(Option<String>),

    /// float
    #[serde(alias = "real")]
    REAL(Option<f64>),

    /// string
    #[serde(alias = "null")]
    NULL(Option<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub field: String,

    pub value: ValueType,

    pub not_null: bool,

    pub primary_key: bool,

    pub autoincrement: bool,
}


#[async_trait]
pub trait Model<T>
    where for<'async_trait> T:
    Model<T>
    + Serialize
    + Debug
    + Clone
    + std::marker::Send
    + 'async_trait
{
    #[allow(dead_code)]
    fn table_name(&self) -> String;

    fn new() -> Self;

    #[allow(dead_code)]
    fn add(&self, _data: &T) -> String {
        let tablename = _data.table_name().clone();
        let data: Value = json!(_data);
        let mut names: Vec<String> = vec![];
        let mut values: Vec<String> = vec![];
        for (key, col_cal) in data.as_object().unwrap() {
            let column: Column = serde_json::from_str(col_cal.as_str().unwrap()).unwrap();
            match column.value {
                ValueType::TEXT(value) => {
                    match value {
                        None => {
                            names.push(key.to_string());
                            values.push(format!("NULL"));
                        }
                        Some(v) => {
                            names.push(key.to_string());
                            values.push(format!("'{}'", v));
                        }
                    }
                }
                ValueType::NULL(_) => {
                    names.push(key.to_string());
                    values.push(format!("NULL"));
                }
                ValueType::INTEGER(value) => {
                    match value {
                        None => {
                            names.push(key.to_string());
                            values.push(format!("0"));
                        }
                        Some(v) => {
                            names.push(key.to_string());
                            values.push(format!("{}", v));
                        }
                    }
                }
                ValueType::BLOB(value) => {
                    match value {
                        None => {
                            names.push(key.to_string());
                            values.push(format!(""));
                        }
                        Some(v) => {
                            names.push(key.to_string());
                            values.push(format!("'{}'", v));
                        }
                    }
                }
                ValueType::REAL(value) => {
                    match value {
                        None => {
                            names.push(key.to_string());
                            values.push(format!("NULL"));
                        }
                        Some(v) => {
                            names.push(key.to_string());
                            values.push(format!("{}", v));
                        }
                    }
                }
            }
        };

        let name_str = names.join(",");

        let value_str = values.join(",");

        format!("INSERT INTO {}({}) VALUES({});", tablename, name_str, value_str)
    }

    #[allow(dead_code)]
    fn adds(&self, models_vec: Vec<T>) -> String {
        let mut statement: String = String::from("");
        for (_index, _item) in models_vec.iter().enumerate() {
            let item: &T = _item;
            statement += &*self.add(item);
        }
        statement
    }

    #[allow(dead_code)]
    fn delete(&self, model_vec: Vec<T>) -> String {
        let mut statement: String = String::from("");
        for (_index, _item) in model_vec.iter().enumerate() {
            let item: Value = json!(_item.clone());
            let mut table: String = String::from("");
            table += &*format!("DELETE FROM {} WHERE ", _item.table_name());
            let mut where_if: Vec<String> = vec![];
            for (key, col_val) in item.as_object().unwrap() {
                let column: Column = serde_json::from_str(col_val.as_str().unwrap()).unwrap();
                match column.value {
                    ValueType::INTEGER(value) => {
                        match value {
                            None => {}
                            Some(v) => where_if.push(format!("{}={}", key, v))
                        }
                    }
                    ValueType::TEXT(value) => {
                        match value {
                            None => {}
                            Some(v) => where_if.push(format!("{}={}", key, v))
                        }
                    }
                    ValueType::BLOB(value) => {
                        match value {
                            None => {}
                            Some(v) => where_if.push(format!("{}={}", key, v))
                        }
                    }
                    ValueType::REAL(value) => {
                        match value {
                            None => {}
                            Some(v) => where_if.push(format!("{}={}", key, v))
                        }
                    }
                    ValueType::NULL(value) => {
                        match value {
                            None => {}
                            Some(v) => where_if.push(format!("{}={}", key, v))
                        }
                    }
                }
            }
            let where_str = where_if.join(" AND ");
            table += &*format!("{};", where_str);
            statement += &*table;
        }
        statement
    }

    #[allow(dead_code)]
    fn update(&self, _model: T, _wrapper: QueryWrapper) -> String {
        todo!()
    }

    #[allow(dead_code)]
    fn select_list(&self, _wrapper: QueryWrapper) -> String {
        String::from("12")
    }
}