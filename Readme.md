# text-ui

There is nothing useful or meaningful here yet.  This is just a vague sketch that I'll probably abandon.

text-ui is a rust library for making interactive text-based applications.

My vague aspiration for this project is an abstract high-level API that can be rendered to multiple backends (plain text, unix terminal, windows terminal, gui).

This library is inspired by the [brick](https://github.com/jtdaugherty/brick) Haskell library.

TODO:
* Finish readline integration
  * History interaction
  * Completion
  * Switch to vi-mode
  * Diff input handling against original to search for places where mid-function reads were dropped
  * Refactor into separate crate, ask previous author if he wants to take it off my hands
* Properly handle full-width characters: https://github.com/unicode-rs/unicode-width
* Widgets
  * Scrolling List
* Work out general focus/input model
* Rewrite with Futures
  * (Sink<Item=Event> + Stream<Frame>)?
  * Optimize rendering by only drawing partial updates from a diff against the previous frame
* Styling