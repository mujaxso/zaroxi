use crate::state::App;
use crate::message::Message;
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    println!("DEBUG: view() called");
    crate::ui::shell::shell(app)
}
