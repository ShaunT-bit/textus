#![warn(clippy::pedantic)] //this has to be later removed while shipping

mod editor;
use editor::Editor;



fn main() {
    Editor::default().run();
}

