extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate pango;

use gdk::ScreenExt; // import get_rgba_visual
use gio::prelude::*;
use gtk::prelude::*;
use gtk::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    set_visual(&window, &None);

    window.fullscreen();
    window.connect_delete_event(quit);
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    window.set_title("Garden");
    window.set_default_size(500, 500);
    window.set_app_paintable(true); // crucial for transparency

    let fixed = Fixed::new();
    window.add(&fixed);

    // 💣💣💣💣
    let map = ". . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .";
    let label = Label::new(map);
    // let text_view = TextView::new();
    // let buffer = text_view.get_buffer().unwrap();
    // buffer.set_text(map);

    let bytelen: u32 = label.get_label().unwrap().len() as u32;

    // !!!!!!!!!!!!!!!
    // Style text view
    // !!!!!!!!!!!!!!!
    let attr_list = pango::AttrList::new();

    // Defaults to black?
    // let mut attr = pango::Attribute::new_background(0, 0, 0)
    //         .expect("Couldn't create new background");
    // attr.set_start_index(0);
    // // attr.set_end_index(7);
    // attr_list.insert(attr);

    let mut attr = pango::Attribute::new_foreground(65535, 65535, 65535)
            .expect("Couldn't create new foreground");
    attr.set_start_index(0);
    attr.set_end_index(bytelen);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_family("'Menlo', monospace")
            .expect("Couldn't create new font family");
    attr.set_start_index(0);
    attr.set_end_index(bytelen);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_scale(2.0)
            .expect("Couldn't create new scale");
    attr.set_start_index(0);
    attr.set_end_index(bytelen);
    attr_list.insert(attr);

    label.set_attributes(&attr_list);

    fixed.add(&label);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.transparent_main_window",
                                            gio::ApplicationFlags::empty())
                                       .expect("Failed to initialize GTK...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}

fn set_visual(window: &ApplicationWindow, _screen: &Option<gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(visual) = screen.get_rgba_visual() {
            window.set_visual(&visual); // crucial for transparency
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // crucial for transparency
    ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    ctx.set_operator(cairo::enums::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}

fn quit(_window: &ApplicationWindow, _event: &gdk::Event) -> Inhibit {
    _window.destroy();
    Inhibit(false)
}
