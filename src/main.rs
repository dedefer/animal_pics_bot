mod image_getter;
mod pic_bot;
mod settings;

use std::error::Error;

use settings::Settings;

use crate::pic_bot::PicBot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    openssl_probe::init_ssl_cert_env_vars();

    let Settings { bot_token, api_token, log_level } = Settings::read()?;
    log::set_max_level(log_level);

    let bot = PicBot::new(bot_token, api_token);

    bot.start().await;

    Ok(())
}
