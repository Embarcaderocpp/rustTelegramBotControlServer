use teloxide::prelude::*;
use tokio::sync::RwLock;
use dotenvy::dotenv;
use std::sync::Arc;


mod handlers;
mod auth;

use auth::{load_allowed_users, AllowedUsers, check_access_filter};
#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting Telegram bot...");

    let bot =  Bot::from_env();

    let allowed_set = load_allowed_users("allowed_users.txt")
        .expect("No file.");

    println!("File load. Load {} users", allowed_set.len());

    let allowed_users: AllowedUsers = Arc::new(RwLock::new(allowed_set));

    let handler = Update::filter_message()
        .filter_async(check_access_filter(allowed_users.clone()))
        .branch(
            dptree::entry()
            .filter_command::<handlers::commands::Command>()
                    .endpoint(handlers::commands::command_handler),
        );


    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![allowed_users])
        .enable_ctrlc_handler()
        .build()
        .dispatch()           
        .await;
}
