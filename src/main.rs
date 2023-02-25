extern crate gtk;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
use gtk::Entry;
use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;

fn main() {
    let app = Application::new(Some("com.example.bissextile"), Default::default()).expect("Initialization failed...");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Bissextile Calculator");
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 100);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        window.add(&vbox);

        let label = Label::new(Some("Enter a year:"));
        vbox.pack_start(&label, false, false, 0);

        let year_input = Entry::new();
        vbox.pack_start(&year_input, false, false, 0);

        let result_label = Label::new(None);
        vbox.pack_start(&result_label, false, false, 0);

        let check_button = Button::with_label("Check");
        check_button.connect_clicked(move |_| {
            let year_str = year_input.get_text().to_string();
            let year = year_str.parse::<i32>().unwrap_or(0);
            if is_bissextile(year) {
                result_label.set_text(&format!("{} is a bissextile year.", year));
            } else {
                result_label.set_text(&format!("{} is not a bissextile year.", year));
            }
        });
        vbox.pack_start(&check_button, false, false, 0);

        window.show_all();
    });

    app.run(&[]);
}

fn is_bissextile(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}