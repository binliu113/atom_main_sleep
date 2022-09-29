use crate::sqlist_model::util::{Model, Column, ValueType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SktListModel {
    pub id: Column,
    pub name: Column,
    pub skt_type: Column,
    pub ip: Column,
    pub port: Column,
    pub online: Column,
}

impl Model<SktListModel> for SktListModel {
    fn table_name(&self) -> String {
        "skt_list".to_string()
    }

    fn new() -> Self {
        Self {
            id: Column {
                field: String::from("id"),
                value: ValueType::INTEGER(None),
                not_null: true,
                primary_key: true,
                autoincrement: true,
            },
            name: Column {
                field: String::from("name"),
                value: ValueType::TEXT(None),
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            skt_type: Column {
                field: String::from("skt_type"),
                value: ValueType::TEXT(None),
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            ip: Column {
                field: String::from("ip"),
                value: ValueType::TEXT(None),
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            port: Column {
                field: String::from("port"),
                value: ValueType::INTEGER(None),
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            online: Column {
                field: String::from("online"),
                value: ValueType::INTEGER(None),
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
        }
    }
}