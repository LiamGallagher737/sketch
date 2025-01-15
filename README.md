<div align="center">

# sketch

 A Rust TUI library

[![crates.io](https://img.shields.io/crates/v/sketch?style=for-the-badge)](https://crates.io/crates/sketch)
[![docs.rs](https://img.shields.io/docsrs/sketch?style=for-the-badge)](https://docs.rs/sketch/latest/sketch)

</div>

## Features

- Fast compile times
- Built on [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform compatibility
- Elm-like [MVU](https://thomasbandt.com/model-view-update) architecture

## Key Concepts

### `Model::update`

The update method processes events by taking a Msg as input. This could be a keyboard key, mouse input, or a custom message. Based on the message, it:

- Updates the model to its next state.
- Optionally returns another message to be processed.

For example:

```rs
fn update(mut self, msg: &Msg) -> (Self, Option<Msg>) {
    // Handle `Key` messages.
    if let Some(key) = msg.cast::<Key>() {
        match key.code {
            // If the enter key was pressed increment the counter.
            KeyCode::Enter => self.count += 1,
            // If the 'q' key was pressed return the `Quit` message to exit the app.
            KeyCode::Char('q') => return (self, Some(Msg::new(Quit))),
            _ => {}
        }
    }

    (self, None)
}
```

### `Model::view`

The view method is responsible for rendering your model into a string. You can use Style to apply formatting:

```rs
fn view(&self) -> String {
    COUNTER_STYLE.render(self.count.to_string())
}
```

### `Message`

A message is any type that implements the `Message` trait. The `Msg` type given to `Model::update` holds a `Message`. The benifet of this approach is messages can be defined by other crates like widget libraries.

A message can be defined like this:

```rs
struct MyMessage {
    something: i32,
}
impl Message for MyMessage {}
```

Your message can then be sent using the app's `Sender` which you can get with `App::sender`:

```rs
let app = App::new(model);
let sender = app.sender();
sender.send(MyMessage { something: 5 });
app.run();
```

For example if you need to make and HTTP request you could spawn a thread to complete the request and then send a message using the sender.

Sketch includes the following messages:

- `Quit`: Send to quit the app.
- `Key`: Keyboard input.
- `Mouse`: Mouse input.
- `Focus`: Focus changes.
- `Paste`: Clipboard pastes. Only if the paste feature is enabeld.
