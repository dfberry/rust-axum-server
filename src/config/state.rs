use std::sync::{Arc, RwLock};
use crate::config::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}