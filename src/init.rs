use std::sync::Arc;
use pumpkin::command::tree::CommandTree;
use pumpkin::plugin::{Context, EventPriority};
use pumpkin::command::tree::builder::literal;
use pumpkin_util::PermissionLvl;
use crate::command::{Login, Register, Changepassword, Bindemail, Resetpassword};
use crate::event::MyJoinHandler;

const NAMES: [&str; 2] = ["CSL", "CatSeedLogin"]; 
const DESCRIPTION: &str = "Login plugin for CatSeed";

pub async fn initialize_plugin(server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();
    log::info!("CatSeedLogin-rust has been loaded!");
    server.register_event(Arc::new(MyJoinHandler), EventPriority::Lowest, true).await;

    let command = CommandTree::new(NAMES, DESCRIPTION) 
        .then(literal("login").execute(Login))
        .then(literal("register").execute(Register))
        .then(literal("changepassword").execute(Changepassword))
        .then(literal("bindemail").execute(Bindemail))
        .then(literal("resetpassword").execute(Resetpassword));
    server.register_command(command, PermissionLvl::Zero).await;
    Ok(())
}
