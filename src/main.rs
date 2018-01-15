extern crate gtk;

use gtk::prelude::*;
use gtk::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.fullscreen();
    window.set_title("First GTK+ Program");

    let title = "hello~";
    let label = Label::new(&*title);
    // label.set_selectable(true);
    // window.add(&label);



    let text_view = TextView::new();
    text_view.get_buffer().unwrap().set_text(title);
    window.add(&text_view);

    // window.set_default_size(350, 70);
    
    // let button = Button::new_with_label("Quit.");
    // window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // button.connect_clicked(|_| {
    //     gtk::main_quit();
    //     // println!("Clicked!");
    //     Inhibit(false);
    // });

    gtk::main();
}
