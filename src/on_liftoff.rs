use rocket::tokio;
use rocket::tokio::sync::mpsc::Receiver;
use rocket_learning::sqlist_model::MODEL_SKT_LIST;
use rocket_learning::sqlist_model::util::Model;

pub struct OnLiftoff {}

impl OnLiftoff {
    async fn sql_handel(_msg: String) {
        let mut skt_list = MODEL_SKT_LIST.lock().await.clone();
        skt_list.id.value = Some(1);
        skt_list.name.value = Some("liubin".to_string());
        let _a = skt_list.delete(vec![skt_list.clone()]).await;
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