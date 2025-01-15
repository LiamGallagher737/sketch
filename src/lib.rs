//! Sketch is a crate for building TUI applications inspired by [bubbletea] from the golang
//! ecosystem.
//!
//! ## Example
//!
//! Here is a simple counter app.
//!
//! ```no_run
//! use sketch::*;
//!
//! const COUNTER_STYLE: Style = Style::new().yellow().bold();
//!
//! fn main() -> std::io::Result<()> {
//!     let model = Counter::default();
//!     App::new(model).run()
//! }
//!
//! #[derive(Default)]
//! struct Counter {
//!     count: usize,
//! }
//!
//! impl Model for Counter {
//!     fn update(mut self, msg: &Msg) -> (Self, Option<Msg>) {
//!         if let Some(key) = msg.cast::<Key>() {
//!             match key.code {
//!                 KeyCode::Enter => self.count += 1,
//!                 KeyCode::Char('q') => return (self, Some(Msg::new(Quit))),
//!                 _ => {}
//!             }
//!         }
//!
//!         (self, None)
//!     }
//!
//!     fn view(&self) -> String {
//!         COUNTER_STYLE.render(self.count.to_string())
//!     }
//! }
//!
//! ```
//!
//! ## [`Model::update`]
//!
//! Whenever something like input happens your model's update function is run with a [`Msg`]. This
//! holds on to the underlying type implementing [`Message`]. By doing it this way you can add your
//! own custom messages and so can libraries providing widgets and other helpers.
//!
//! The point of this function is to use this [`Msg`] to create a new model to be used for the next
//! render. The function can also optionally return another [`Msg`]. If another message is returned
//! it will be given to [`Model::update`] and continue to run them until a message is not returned.
//! The app will render once all returned messages are run.
//!
//! ## [`Model::view`]
//!
//! This is where you render your model into a string to be displayed to the user. To make your ap
//! look nice you can use the [`Style`] struct to render text with different attributes.
//!
//! ## [`Model::startup`]
//!
//! This function runs on startup and if a message is returned it will be run as the first message
//! for [`Model::update`].
//!
//! ## Built-in messages
//!
//! The following are the built-in messages.
//!
//! * [`Quit`]: Send to quit the app.
//! * [`Key`]: Keyboard input.
//! * [`Mouse`]: Mouse input.
//! * [`Focus`]: Focus changes.
//! * [`Paste`]: Clipboard pastes. Only if the `paste` feature is enabeld.
//!
//! ## Custom messages
//!
//! Any type that is [`Send`] can be made in to a [`Message`] by implementing it.
//!
//! ```
//! # use sketch::*;
//! struct MyMessage {
//!     something: i32,
//! }
//! impl Message for MyMessage {}
//!
//! ```
//!
//! You can then send them using a [`Sender<Msg>`] from [`App::sender`].
//!
//! For example if you need to make and HTTP request you could spawn a thread to complete the
//! request and then send a message using the sender.
//!
//! [bubbletea]: https://github.com/charmbracelet/bubbletea

#![deny(missing_docs)]

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
    io::{self, Write},
    sync::mpsc::{channel, Receiver, Sender},
};

pub use msg::*;
pub use style::*;

mod msg;
mod style;

/// A type to hold on to and run your [`Model`].
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
            let view = self.model.view().replace("\n", "\r\n");
            // TODO: Diff this and last frame and only update what has changed.
            execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Print(&view))?;
            stdout.flush()?;

            let mut m = Some(self.message_receiver.recv().unwrap());
            while let Some(msg) = m {
                if msg.is::<Quit>() {
                    break 'outer;
                }

                let out = self.model.update(&msg);
                self.model = out.0;
                m = out.1;
            }
        }

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }
}

/// A trait to turn your data in to something [`App`] can run.
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
