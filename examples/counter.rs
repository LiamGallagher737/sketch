use sketch::*;

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
                KeyCode::Enter => self.count += 1,
                KeyCode::Char('q') => return (self, Some(Msg::new(Quit))),
                _ => {}
            }
        }

        (self, None)
    }

    fn view(&self) -> String {
        COUNTER_STYLE.render(self.count.to_string())
    }
}
