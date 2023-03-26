use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use self::todos::Todo;

mod todos;

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    let db = Db::default();

    let app = todos::router(db);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
