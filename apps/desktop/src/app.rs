mod message;
mod state;
mod update;
mod view;

pub use message::Message;
pub use state::{App, Activity, FileLoadingState, FileMetadata};
pub use update::update;
pub use view::view;

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
