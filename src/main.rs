mod image_getter;
mod pic_bot;
mod settings;

use std::error::Error;

use lazy_static::lazy_static;
use settings::Settings;

use crate::pic_bot::PicBot;

lazy_static! {
    static ref CONFIG: Settings = match Settings::read() {
        Ok(cfg) => cfg,
        Err(e) => panic!("cant read config: {}", e),
    };

    static ref BOT: PicBot = PicBot::new(
        CONFIG.bot_token.clone(),
        CONFIG.api_token.clone(),
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    openssl_probe::init_ssl_cert_env_vars();

    log::set_max_level(CONFIG.log_level);

    BOT.start().await;
    Ok(())
}
