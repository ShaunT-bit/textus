#![warn(clippy::all, clippy::pedantic, clippy::print_stdout, clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)] //this has to be later removed while shipping

mod editor;
use editor::Editor;



fn main() {
    Editor::default().run();
}

