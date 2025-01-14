use sketch::{Key, KeyCode, Msg, Quit, Style};

const TEXT_STYLE: Style = Style::new();

fn main() -> std::io::Result<()> {
    let model = Model::default();
    sketch::App::new(model).run()
}

#[derive(Debug, Default)]
struct Model {
    text: String,
    cursor: usize,
}

impl sketch::Model for Model {
    fn update(mut self, msg: &Msg) -> (Self, Option<Msg>) {
        if let Some(key) = msg.cast::<Key>() {
            match key.code {
                KeyCode::Char('c') if key.with_control() => return (self, Some(Msg::new(Quit))),
                KeyCode::Char(c) => {
                    self.text.push(c);
                    self.cursor += 1;
                }
                KeyCode::Backspace if self.cursor > 0 => {
                    self.text.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
                KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
                KeyCode::Right if self.cursor < self.text.len() - 1 => self.cursor += 1,
                KeyCode::Enter => {
                    self.text.push('\n');
                    self.cursor += 1
                }
                _ => {}
            }
        }

        (self, None)
    }

    fn view(&self) -> String {
        TEXT_STYLE.render(&self.text)
    }
}
