extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate pango;
extern crate chrono;

use gdk::ScreenExt; // import get_rgba_visual
use gio::prelude::*;
use gio::MenuExt;
use gtk::prelude::*;
use gtk::*;
use chrono::Local;

use std::env::args;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn render_map() -> String {
    format!(
        "{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}\n{0} {0} {0} {0} {0} {0} {0}",
        Local::now().timestamp() % 10,
    )
}

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

    let grid = Grid::new();
    window.add(&grid);

    // 💣💣💣💣
    let map = ". . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .\n. . . . . . .";
    let label = Label::new(map);
    grid.add(&label);

    label.set_vexpand(true);
    label.set_hexpand(true);

    // !!!!!!!!!!!!!!!
    // Style text view
    // !!!!!!!!!!!!!!!
    let attr_list = pango::AttrList::new();

    // Get the byte-length (indexes, not character offsets) of the map string buffer.
    let bytelen: u32 = label.get_label().unwrap().len() as u32;

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

    let mut attr = pango::Attribute::new_family("monospace")
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

    // Animate text label contents.
    let tick = move || {
        let map = render_map();
        label.set_text(&map);
        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);

    add_menu(application);

    add_actions(application, &window);

    window.show_all();
}

fn add_menu(app: &gtk::Application) {
    let menu = gio::Menu::new();
    let about = gio::Menu::new();
    about.append("About", "app.about");
    let quit = gio::Menu::new();
    quit.append("Quit", "app.quit");

    menu.append_section("About", &about);
    menu.append_section("Quit", &quit);

    app.set_app_menu(&menu);
}

fn add_actions(app: &gtk::Application, window: &gtk::ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(window => move |_, _| {
        window.destroy();
    }));

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(window => move |_, _| {
        let p = AboutDialog::new();
        p.set_title("About");
        p.set_program_name("Tick");
        p.set_comments(Some("Made by mtso"));
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    }));

    app.add_action(&quit);
    app.add_action(&about);
}

fn main() {
    let application = gtk::Application::new("io.mtso.textgrid",
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
