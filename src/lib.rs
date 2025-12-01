use std::sync::Arc;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin::plugin::Context;

#[plugin_method] 
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!(); 
    log::info!("CatSeedLogin-rust has been loaded!");
    Ok(())
}

#[plugin_impl]
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