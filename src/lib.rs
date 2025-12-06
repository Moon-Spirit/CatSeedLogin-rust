mod command;
mod event;
mod init;

use std::sync::Arc;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin::plugin::Context;
use crate::init::initialize_plugin;

#[pumpkin_api_macros::plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    initialize_plugin(server).await
}

#[pumpkin_api_macros::plugin_impl]
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
