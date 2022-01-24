use adw::{prelude::*, ApplicationWindow, HeaderBar, SplitButton};
use gtk::{
    prelude::*, ActionBar, Adjustment, Align, Application, AspectFrame, Box, Button, Grid, Image,
    Orientation, Scale, Separator, ToggleButton,
};

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
        //////////////////
        // MAIN CONTENT //
        //////////////////

        let focus_image = Image::builder()
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .vexpand(true)
            .hexpand(true)
            .build();

        let focus_neighbours_grid = Grid::builder()
            .vexpand(true)
            .hexpand(true)
            .column_spacing(0)
            .row_spacing(0)
            .build();
        let focus_neighbours_aspect_frame = AspectFrame::builder()
            .child(&focus_neighbours_grid)
            .ratio(1.0)
            .xalign(0.5)
            .yalign(0.5)
            .build();

        let neighbours_1 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_2 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_3 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_4 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_5 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_6 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_7 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_8 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();
        let neighbours_9 = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
            .build();

        //focus_neighbours_grid.add

        focus_neighbours_grid.attach(&neighbours_1, 0, 0, 1, 1);
        focus_neighbours_grid.attach(&neighbours_2, 1, 0, 1, 1);
        focus_neighbours_grid.attach(&neighbours_3, 2, 0, 1, 1);
        focus_neighbours_grid.attach(&neighbours_4, 0, 1, 1, 1);
        focus_neighbours_grid.attach(&neighbours_5, 1, 1, 1, 1);
        focus_neighbours_grid.attach(&neighbours_6, 2, 1, 1, 1);
        focus_neighbours_grid.attach(&neighbours_7, 0, 2, 1, 1);
        focus_neighbours_grid.attach(&neighbours_8, 1, 2, 1, 1);
        focus_neighbours_grid.attach(&neighbours_9, 2, 2, 1, 1);

        let focus_scale_adjustment = Adjustment::builder()
            .lower(0.0)
            .upper(10.0)
            .value(5.0)
            .step_increment(1.0)
            .build();

        let focus_scale = Scale::builder()
            .orientation(Orientation::Vertical)
            .adjustment(&focus_scale_adjustment)
            .vexpand(true)
            .margin_top(MARGIN_TOP)
            .margin_bottom(MARGIN_BOTTOM)
            .margin_start(MARGIN_LEFT / 2)
            .margin_end(MARGIN_RIGHT / 2)
            .draw_value(true)
            .inverted(true)
            .round_digits(0)
            .digits(0)
            .build();

        let center_content_seperator = Separator::new(Orientation::Vertical);
        let center_content = Box::builder()
            //.hexpand(true)
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        center_content.append(&focus_scale);
        center_content.append(&center_content_seperator);
        //center_content.append(&focus_image);
        center_content.append(&focus_neighbours_aspect_frame);

        focus_scale.connect_value_changed(move |x| {
            eprintln!("Changed value! {:?}", x.value());
            let path = if x.value() > 6.0 {
                "/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg"
            } else if x.value() > 3.0 {
                "/var/home/hannes/Downloads/test/I12984_X022_Y029_Z5146.jpg"
            } else {
                "/var/home/hannes/Downloads/test/I12985_X022_Y029_Z5195.jpg"
            };
            focus_image.set_from_file(Some(path));
        });

        ////////////
        // HEADER //
        ////////////

        //let show_start_title_buttons = Button::new();
        let header_bar = HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("First App", ""))
            .build();

        // TODO: add button functionality
        let open_button = SplitButton::builder().label("Open").build();
        header_bar.pack_start(&open_button);

        ////////////////////
        // BOTTOM TOOLBAR //
        ///////////////////

        let bottom_toolbar = ActionBar::builder().build();

        // TODO: add functionality
        let skip_button = Button::builder().label("Skip").build();
        let focus_button = Button::builder()
            .label("Set Focus")
            .css_classes(vec!["suggested-action".to_string()])
            .build();
        let focus_skip_link_widget = Box::builder()
            .css_classes(vec!["linked".to_string()])
            .build();
        focus_skip_link_widget.append(&skip_button);
        focus_skip_link_widget.append(&focus_button);

        let neighbour_toggle_button = ToggleButton::builder().label("Toggle Neighbours").build();

        bottom_toolbar.pack_start(&neighbour_toggle_button);
        bottom_toolbar.pack_end(&focus_skip_link_widget);

        //////////////////////
        // MAIN APPLICATION //
        //////////////////////

        // Combine the content in a box
        let application_vertical_widget = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar

        application_vertical_widget.append(&header_bar);
        application_vertical_widget.append(&center_content);
        application_vertical_widget.append(&bottom_toolbar);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            // add content to window
            .content(&application_vertical_widget)
            .build();
        window.show();
    });

    application.run();
}
