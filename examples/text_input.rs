use crossterm::event::KeyCode;
use sketch::{msg, Msg};

fn main() -> std::io::Result<()> {
    let model = Model::default();
    sketch::App::new(model).run()
}

#[derive(Debug, Default, Clone)]
struct Model {
    text: String,
    cursor: usize,
}

impl sketch::Model for Model {
    fn update(mut self, msg: &Msg) -> (Self, Option<Msg>) {
        if let Some(key) = msg.cast::<msg::Key>() {
            match key.code {
                KeyCode::Char('c') if key.with_control() => {
                    return (self, Some(Msg::new(msg::Quit)))
                }
                KeyCode::Char(c) => {
                    self.text.push(c);
                    self.cursor += 1;
                }
                KeyCode::Backspace => {
                    self.text.remove(self.cursor - 1);
                    self.cursor = self.cursor.saturating_sub(1);
                }
                KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
                KeyCode::Right if self.cursor < self.text.len() - 1 => self.cursor += 1,
                _ => {}
            }
        }

        (self, None)
    }

    fn view(&self) -> String {
        self.text.clone()
    }
}
