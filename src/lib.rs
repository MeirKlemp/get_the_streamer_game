mod interface;
mod sprites;

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, Context, GameResult};
use interface::Interface;
use sprites::Sprites;
use std::sync::mpsc::{Receiver, Sender};
use twitch_chat_wrapper::chat_message::ChatMessage;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    interface: Interface,
    sprites: Sprites,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
        context: &mut Context,
    ) -> GameResult<GameState> {
        let screen_size = graphics::drawable_size(context);
        let interface = Interface::new(context, screen_size)?;
        let sprites = Sprites::new(context)?;

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            interface,
            sprites,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        if let Ok(chat_message) = self.receive_from_chat.try_recv() {
            dbg!(chat_message);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        let screen_size = graphics::drawable_size(context);
        self.interface.draw(context, screen_size)?;

        graphics::draw(context, &self.sprites.fire, graphics::DrawParam::new())?;

        graphics::present(context)
    }
}
