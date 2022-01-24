use adw::prelude::*;
use gtk::prelude::*;

use adw::{ApplicationWindow, HeaderBar};
use gtk::{Adjustment, Application, Box, Image, Orientation, Scale};

const MARGIN_TOP: i32 = 32;
const MARGIN_BOTTOM: i32 = 32;
const MARGIN_LEFT: i32 = 32;
const MARGIN_RIGHT: i32 = 32;

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        let adjustment = Adjustment::builder()
            .lower(0.0)
            .upper(10.0)
            .value(5.0)
            .step_increment(1.0)
            .build();

        let image = Image::builder()
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .vexpand(true)
            .hexpand(true)
            .margin_top(MARGIN_TOP)
            .margin_end(MARGIN_RIGHT)
            .margin_bottom(MARGIN_BOTTOM)
            .margin_start(MARGIN_LEFT)
            .build();

        let scale = Scale::builder()
            .orientation(Orientation::Vertical)
            .adjustment(&adjustment)
            .vexpand(true)
            .margin_top(MARGIN_TOP)
            .margin_bottom(MARGIN_BOTTOM)
            .margin_start(MARGIN_LEFT)
            .draw_value(true)
            .inverted(true)
            .round_digits(0)
            .digits(0)
            //(|x| eprintln!("Changed! {:?}", x))
            .build();

        let content = Box::builder()
            //.hexpand(true)
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        content.append(&scale);
        content.append(&image);

        scale.connect_value_changed(move |x| {
            eprintln!("Changed value! {:?}", x.value());
            let path = if x.value() > 6.0 {
                "/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg"
            } else if x.value() > 3.0 {
                "/var/home/hannes/Downloads/test/I12984_X022_Y029_Z5146.jpg"
            } else {
                "/var/home/hannes/Downloads/test/I12985_X022_Y029_Z5195.jpg"
            };
            image.set_from_file(Some(path));
        });

        //let show_start_title_buttons = Button::new();
        let header_bar = HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("First App", ""))
            .build();

        // Combine the content in a box
        let title_widget_content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar

        title_widget_content.append(&header_bar);
        title_widget_content.append(&content);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            // add content to window
            .content(&title_widget_content)
            .build();
        window.show();
    });

    application.run();
}
