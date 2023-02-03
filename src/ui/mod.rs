use fltk::{prelude::*, *};

pub(crate) fn build_ui() {
    let mut buf = text::TextBuffer::default();

    let mut win = window::Window::new(100, 100, 500, 500, "ride");
    win.make_resizable(true);
    let mut txt = text::TextEditor::default()
        .with_size(390, 290)
        .size_of_parent();
    buf.set_text("Hello world!");
    buf.append("\n");
    buf.append("This is a text editor!");
    txt.set_buffer(buf);
    win.end();
    win.show();
}
