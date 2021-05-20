use std::error::Error;

use teloxide::{Bot, prelude::{Request, UpdateWithCx}, types::{InputFile, Message}};

use crate::image_getter::ImageGetter;

pub struct PicBot {
    bot: Bot,
    image_getter: ImageGetter,
}

impl PicBot {
    pub fn new(bot_token: String, image_api_token: String) -> PicBot {
        PicBot {
            bot: Bot::new(bot_token),
            image_getter: ImageGetter::new(image_api_token),
        }
    }

    pub async fn start(&'static self) {
        teloxide::enable_logging!();

        log::info!("starting bot");

        teloxide::repl(
            self.bot.clone(),
            move |m| self.answer(m),
        ).await
    }

    async fn answer(&'static self, message: UpdateWithCx<Bot, Message>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let text = match message.update.text() {
            Some(t) => t,
            None => {
                message.answer("send me text query pls").send().await?;
                return Ok(())
            },
        };

        let url = match self.image_getter.get_image_url(text).await? {
            Some(url) => url,
            None => {
                message.answer(format!("no images for query {:?}", text)).send().await?;
                return Ok(())
            },
        };

        message.answer_photo(InputFile::url(url)).send().await?;

        Ok(())
    }
}
