use rocket::tokio;
use rocket::tokio::sync::mpsc::Receiver;
use rocket_atom::sqlist_model::{
    util::Model,
    wrapper::{QueryWrapper},
    skt_list_model::SktListModel,
};
use rocket_atom::sqlist_model::util::ValueType;


pub struct OnLiftoff {}

impl OnLiftoff {
    async fn sql_handel(_msg: String) {
        let skt_list = SktListModel::new();
        let mut wrp: QueryWrapper = QueryWrapper::new();
        let name_val = Some(String::from(""));
        wrp.eq(&skt_list.name, ValueType::TEXT(name_val.clone()))
            .and()
            .eq(&skt_list.name, ValueType::TEXT(name_val.clone()));

        wrp.debug();
    }

    pub async fn sql_event(mut rx: Receiver<String>) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = rx.recv() => {
                        OnLiftoff::sql_handel(msg).await;
                    },
                }
            }
        });
    }
}