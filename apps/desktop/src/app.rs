// Re-export the main types from the crate root modules
pub use crate::message::Message;
pub use crate::state::{App, Activity, FileLoadingState, FileMetadata};
pub use crate::update::update;
pub use crate::view::view;

use iced::{Element, Command};

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        App::new()
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.subscription()
    }
}
