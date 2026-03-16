use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Client>,
    pub auth: super::auth::Auth,
}
