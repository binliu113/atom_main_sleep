use std::fmt::{Debug};
use async_trait::async_trait;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use super::super::sqlist_model::{SQLITE_CONF, SQLITE_MODEL};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Type {
    #[serde(alias = "integer")]
    INTEGER,

    #[serde(alias = "text")]
    TEXT,

    #[serde(alias = "int")]
    INT,

    #[serde(alias = "blob")]
    BLOB,

    #[serde(alias = "real")]
    REAL,

    #[serde(alias = "any")]
    ANY,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column<F> {
    pub value: Option<F>,

    pub col_type: Type,

    pub not_null: bool,

    pub primary_key: bool,

    pub autoincrement: bool,
}


async fn execute(query: String) -> bool {
    let debug_query = SQLITE_CONF.debug_query.unwrap();
    match SQLITE_MODEL.lock().await.conn.execute(query.clone()) {
        Ok(_) => {
            if debug_query {
                log::info!(
                    "[36m{:^6}[0m",
                    query.clone()
                );
            }
            true
        }
        Err(e) => {
            log::error!(
                "[31m{:^6} | Err: “{:^6}” |[0m",
                query.clone(),
                e.message.unwrap()
            );
            false
        }
    }
}


#[async_trait]
pub trait Model<T>
    where for<'async_trait> T: Model<T>
    + Serialize
    + Debug
    + Clone
    + std::marker::Send + 'async_trait
{
    fn table_name(&self) -> String;

    fn new() -> Self;

    async fn add(&self, _data: T) -> bool {
        let _data: Value = json!(_data.clone());

        true
    }

    async fn adds(&self, _datas: Vec<T>) -> bool {
        todo!()
    }

    async fn delete(&self, model_vec: Vec<T>) -> bool {
        let mut statement: String = String::from("");
        for (_index, _item) in model_vec.iter().enumerate() {
            let item: Value = json!(_item.clone());
            let mut table: String = String::from("");
            table += &*format!("DELETE FROM {} WHERE ", _item.table_name());
            let mut where_if: Vec<String> = vec![];
            for (key, col_val) in item.as_object().unwrap() {
                match col_val["value"].clone() {
                    Value::Bool(v) => where_if.push(format!("{}={}", key, v)),
                    Value::Number(v) => where_if.push(format!("{}={}", key, v)),
                    Value::String(v) => where_if.push(format!("{}='{}'", key, v)),
                    _ => {}
                }
            }
            let where_str = where_if.join(" AND ");

            table += &*format!("{};", where_str);
            statement += &*table;
        }
        execute(statement.clone()).await
    }

    async fn update(&self) -> bool {
        todo!()
    }

    fn find_all_page(_psize: u32, _page_num: u32) -> Vec<T> {
        todo!()
    }
}