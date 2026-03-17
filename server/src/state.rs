use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio_postgres::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub rt: Arc<Runtime>,
    pub auth: super::auth::Auth,
}
