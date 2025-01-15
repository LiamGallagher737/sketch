use sketch::*;

const TITLE_STYLE: Style = Style::new().bold();
const COUNTER_STYLE: Style = Style::new().yellow().bold();

fn main() -> std::io::Result<()> {
    let model = Counter::default();
    App::new(model).run()
}

#[derive(Default)]
struct Counter {
    count: usize,
}

impl Model for Counter {
    fn update(mut self, msg: &Msg) -> (Self, Option<Msg>) {
        if let Some(key) = msg.cast::<Key>() {
            match key.code {
                KeyCode::Enter if key.is_press() => self.count += 1,
                KeyCode::Char('q') => return (self, Some(Msg::new(Quit))),
                KeyCode::Char('c') if key.with_control() => return (self, Some(Msg::new(Quit))),
                _ => {}
            }
        }

        (self, None)
    }

    fn view(&self) -> String {
        let mut s = String::new();
        let (_, rows) = terminal_size().unwrap();

        s.push_str(&"\n".repeat(rows as usize / 2));

        let content = format!(
            "{} {}",
            &TITLE_STYLE.render("Count:"),
            &COUNTER_STYLE.render(self.count.to_string())
        );
        s.push_str(&Style::new().center().render(content));

        s
    }
}
