use std::sync::Arc;
use taskgeneral_core::TaskManagerWrapper;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub manager: Arc<TaskManagerWrapper>,
}
