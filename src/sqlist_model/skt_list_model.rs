use crate::sqlist_model::util::{Model, Column, Type};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SktListModel {
    pub id: Column<u32>,
    pub name: Column<String>,
    pub skt_type: Column<String>,
    pub ip: Column<String>,
    pub port: Column<u32>,
    pub online: Column<u32>,
}

impl Model<SktListModel> for SktListModel {
    fn table_name(&self) -> String {
        "skt_list".to_string()
    }

    fn new() -> Self {
        Self {
            id: Column {
                value: None,
                col_type: Type::INTEGER,
                not_null: true,
                primary_key: true,
                autoincrement: true,
            },
            name: Column {
                value: None,
                col_type: Type::TEXT,
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            skt_type: Column {
                value: None,
                col_type: Type::TEXT,
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            ip: Column {
                value: None,
                col_type: Type::TEXT,
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            port: Column {
                value: None,
                col_type: Type::INTEGER,
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
            online: Column {
                value: None,
                col_type: Type::INTEGER,
                not_null: true,
                primary_key: false,
                autoincrement: false,
            },
        }
    }
}