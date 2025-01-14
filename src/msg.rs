use std::any::Any;

use crossterm::event::{
    KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

pub use crossterm::event::KeyCode;

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

macro_rules! matches_method {
    ($method:ident, $field:ident, $value:pat) => {
        pub fn $method(&self) -> bool {
            matches!(self.$field, $value)
        }
    };
}

macro_rules! bitflags_method {
    ($method:ident, $field:ident, $flags:ident, $constant:ident) => {
        pub fn $method(&self) -> bool {
            self.$field.contains($flags::$constant)
        }
    };
}

/// A message to instruct the [`App`](crate::App) to exit.
#[derive(Debug)]
pub struct Quit;
impl Message for Quit {}

/// A message keyboard input.
#[derive(Debug)]
pub struct Key {
    pub code: KeyCode,
    modifiers: KeyModifiers,
    kind: KeyEventKind,
    state: KeyEventState,
}
impl Message for Key {}

impl Key {
    matches_method! { is_press, kind, KeyEventKind::Press }
    matches_method! { is_release, kind, KeyEventKind::Release }
    matches_method! { is_repeat, kind, KeyEventKind::Repeat }
    bitflags_method! { with_shift, modifiers, KeyModifiers, SHIFT }
    bitflags_method! { with_control, modifiers, KeyModifiers, CONTROL }
    bitflags_method! { with_alt, modifiers, KeyModifiers, ALT }
    bitflags_method! { with_super, modifiers, KeyModifiers, SUPER }
    bitflags_method! { with_hyper, modifiers, KeyModifiers, HYPER }
    bitflags_method! { with_meta, modifiers, KeyModifiers, META }
    bitflags_method! { from_keypad, state, KeyEventState, KEYPAD }
    bitflags_method! { with_capslock, state, KeyEventState, CAPS_LOCK }
    bitflags_method! { with_numlock, state, KeyEventState, NUM_LOCK }
}

impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
            kind: value.kind,
            state: value.state,
        }
    }
}

/// A message for mouse input.
#[derive(Debug)]
pub struct Mouse {
    kind: MouseEventKind,
    modifiers: KeyModifiers,
    pub column: u16,
    pub row: u16,
}
impl Message for Mouse {}

impl Mouse {
    pub fn is_left(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Down(MouseButton::Left)
                | MouseEventKind::Up(MouseButton::Left)
                | MouseEventKind::Drag(MouseButton::Left)
        )
    }

    pub fn is_right(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Down(MouseButton::Right)
                | MouseEventKind::Up(MouseButton::Right)
                | MouseEventKind::Drag(MouseButton::Right)
        )
    }

    pub fn is_middle(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Down(MouseButton::Middle)
                | MouseEventKind::Up(MouseButton::Middle)
                | MouseEventKind::Drag(MouseButton::Middle)
        )
    }

    pub fn is_scroll(&self) -> bool {
        use MouseEventKind::*;
        matches!(self.kind, ScrollUp | ScrollDown | ScrollLeft | ScrollRight)
    }

    matches_method! { is_press, kind, MouseEventKind::Down(_) }
    matches_method! { is_release, kind, MouseEventKind::Up(_) }
    matches_method! { is_drag, kind, MouseEventKind::Drag(_) }
    matches_method! { is_move, kind, MouseEventKind::Moved }
    matches_method! { is_scroll_up, kind, MouseEventKind::ScrollUp }
    matches_method! { is_scroll_down, kind, MouseEventKind::ScrollDown }
    matches_method! { is_scroll_left, kind, MouseEventKind::ScrollLeft }
    matches_method! { is_scroll_right, kind, MouseEventKind::ScrollRight }
    bitflags_method! { with_shift, modifiers, KeyModifiers, SHIFT }
    bitflags_method! { with_control, modifiers, KeyModifiers, CONTROL }
    bitflags_method! { with_alt, modifiers, KeyModifiers, ALT }
    bitflags_method! { with_super, modifiers, KeyModifiers, SUPER }
    bitflags_method! { with_hyper, modifiers, KeyModifiers, HYPER }
    bitflags_method! { with_meta, modifiers, KeyModifiers, META }
}

impl From<MouseEvent> for Mouse {
    fn from(value: MouseEvent) -> Self {
        Self {
            kind: value.kind,
            column: value.column,
            row: value.row,
            modifiers: value.modifiers,
        }
    }
}

/// A message for terminal focus.
#[derive(Debug)]
pub enum Focus {
    Gained,
    Lost,
}
impl Message for Focus {}

/// A message for user pasting from clipboard.
#[cfg(feature = "paste")]
pub struct Paste(pub String);
#[cfg(feature = "paste")]
impl Message for Paste {}

/// A message for terminal window resizing.
pub struct Resize {
    /// The number of columns available.
    pub width: u16,
    /// The number of rows available.
    pub height: u16,
}
impl Message for Resize {}
