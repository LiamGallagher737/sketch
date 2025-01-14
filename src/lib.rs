use crossterm::{
    cursor::MoveTo,
    event::{self, Event},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    any::Any,
    io::{self, Write},
    sync::mpsc::{channel, Receiver, Sender},
};

pub use msg::*;
pub use style::*;

mod msg;
mod style;

pub struct App<M: Model> {
    model: M,
    message_sender: Sender<Msg>,
    message_receiver: Receiver<Msg>,
}

impl<M: Model> App<M> {
    /// Create a new [`App`].
    #[must_use = "Creating an app does nothing until you call App::run()"]
    pub fn new(model: M) -> Self {
        let (message_sender, message_receiver) = channel();
        Self {
            model,
            message_sender,
            message_receiver,
        }
    }

    /// Get a copy of the [`Sender`] for sending [`Msg`]s.
    pub fn sender(&self) -> Sender<Msg> {
        self.message_sender.clone()
    }

    /// Run this [`App`] only returning once the [`Quit`] message has been sent.
    pub fn run(mut self) -> std::io::Result<()> {
        set_panic_hook();
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        spawn_crossterm_event_thread(self.message_sender.clone());

        if let Some(msg) = self.model.startup() {
            self.message_sender.send(msg).unwrap();
        }

        'outer: loop {
            let mut m = Some(self.message_receiver.recv().unwrap());
            while let Some(msg) = m {
                if msg.is::<Quit>() {
                    break 'outer;
                }

                let out = self.model.update(&msg);
                self.model = out.0;
                m = out.1;
            }

            let view = self.model.view().replace("\n", "\r\n");
            // TODO: Diff this and last frame and only update what has changed.
            execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Print(&view))?;
            stdout.flush()?;
        }

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }
}

pub trait Model: Sized {
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
    msg: Box<dyn Any + Send>,
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

/// A trait to allow a type to be used as a [`Msg`].
pub trait Message: Send {}

fn spawn_crossterm_event_thread(tx: Sender<Msg>) {
    std::thread::spawn(move || loop {
        let msg = match event::read().expect("Failed to read crossterm event") {
            Event::FocusGained => Msg::new(Focus::Gained),
            Event::FocusLost => Msg::new(Focus::Lost),
            Event::Key(event) => Msg::new(Key::from(event)),
            Event::Mouse(event) => Msg::new(Mouse::from(event)),
            Event::Resize(width, height) => Msg::new(Resize { width, height }),

            #[cfg(feature = "paste")]
            Event::Paste(value) => Msg::new(msg::Paste(value)),
            #[cfg(not(feature = "paste"))]
            Event::Paste(_) => continue,
        };

        tx.send(msg).expect("Failed to send on message channel");
    });
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        hook(info);
    }));
}
