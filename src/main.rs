#![warn(clippy::pedantic)] //this has to be later removed while shipping

mod editor;
use editor::Editor;



fn main() {
    let mut editor = Editor::default();
    editor.run();
}

