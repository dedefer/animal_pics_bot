mod image_getter;
mod pic_bot;
mod settings;

use std::error::Error;

use settings::Settings;
use tokio::spawn;

use crate::pic_bot::PicBot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    openssl_probe::init_ssl_cert_env_vars();

    let config = Settings::read()?;
    log::set_max_level(config.log_level);

    let bot = PicBot::new(config.bot_token, config.api_token);

    spawn(bot.start());

    Ok(())
}
