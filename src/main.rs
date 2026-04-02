use teloxide::prelude::*;
use dotenvy::dotenv;

mod handlers;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting Telegram bot...");

    let bot =  Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::entry()
            .filter_command::<handlers::commands::Command>()
                    .endpoint(handlers::commands::command_handler),
        );


    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()           
        .await;
}
