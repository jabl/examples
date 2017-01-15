//! # gtk::Application example
//!
//! Simple example using the gtk::Application as the top-level
//! application class.

#![crate_type = "bin"]

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::Button;

fn main() {

    let app = gtk::Application::new("gtk-rs.examples.gtkapplication",
                                    gio::APPLICATION_FLAGS_NONE).unwrap();

    app.connect_activate(move |app| {
        let widget = gtk::ApplicationWindow::new(&app);
        let button = Button::new_with_label("Click me");
        widget.add(&button);
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        widget.show_all();
    });
                         
    let a: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = a.iter().map(|x| x.as_str()).collect();
    
    app.run(args.len() as i32, &args);
}

