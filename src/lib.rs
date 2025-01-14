use crossterm::{
    cursor::MoveToColumn,
    event::{self, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use msg::{Focus, Key, Mouse, Quit, Resize};
use std::{
    any::Any,
    io::{self, Write},
    sync::mpsc::{channel, Receiver, Sender},
};

pub mod msg;

pub struct App<M: Model> {
    model: M,
    message_sender: Sender<Msg>,
    message_receiver: Receiver<Msg>,
}

impl<M: Model> App<M> {
    #[must_use = "Creating an app does nothing until you call App::run()"]
    pub fn new(model: M) -> Self {
        let (message_sender, message_receiver) = channel();
        Self {
            model,
            message_sender,
            message_receiver,
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        set_panic_hook();
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let mut message = self.model.startup();
        'outer: loop {
            while let Some(msg) = &message {
                if msg.is::<Quit>() {
                    break 'outer;
                }

                let out = self.model.update(msg);
                self.model = out.0;
                message = out.1;
            }

            let render = self.model.view();
            execute!(stdout, Clear(ClearType::CurrentLine))?;
            execute!(stdout, MoveToColumn(0))?;
            print!("{render}");
            stdout.flush()?;

            message = Some(match event::read()? {
                Event::FocusGained => Msg::new(Focus::Gained),
                Event::FocusLost => Msg::new(Focus::Lost),
                Event::Key(event) => Msg::new(Key::from(event)),
                Event::Mouse(event) => Msg::new(Mouse::from(event)),
                Event::Resize(width, height) => Msg::new(Resize { width, height }),

                #[cfg(feature = "paste")]
                Event::Paste(value) => Msg::new(msg::Paste(value)),
                #[cfg(not(feature = "paste"))]
                Event::Paste(_) => continue,
            });
        }

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }
}

pub trait Model: Clone + Sized {
    /// Where any initial startup commands are sent.
    fn startup(&self) -> Option<Msg> {
        None
    }

    /// Where the messages are used to construct a new model.
    fn update(self, msg: &Msg) -> (Self, Option<Msg>);

    /// Where the model is used to render a frame.
    fn view(&self) -> String;
}

pub struct Msg {
    msg: Box<dyn Any>,
}

impl Msg {
    /// Create a new [`Msg`] from a type implementing [`Message`].
    pub fn new<M: Message + 'static>(msg: M) -> Self {
        Self { msg: Box::new(msg) }
    }

    /// Try convert this [`Msg`] to a explicit [`Message`] implementing type.
    pub fn cast<M: Message + 'static>(&self) -> Option<&M> {
        self.msg.downcast_ref::<M>()
    }

    /// Check if this [`Msg`] is a specific [`Message`] implementing type.
    pub fn is<M: Message + 'static>(&self) -> bool {
        self.msg.is::<M>()
    }
}

pub trait Message {}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        hook(info);
    }));
}
