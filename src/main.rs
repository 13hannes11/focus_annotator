use adw::{prelude::*, ApplicationWindow, HeaderBar, SplitButton};
use gio::SimpleAction;
use glib::clone;
use gtk::{gio, glib};
use gtk::{
    prelude::*, ActionBar, Adjustment, Application, AspectFrame, Box, Button, Grid, Image,
    Orientation, PositionType, Scale, Separator, ToggleButton,
};

const MARGIN_TOP: i32 = 32;
const MARGIN_BOTTOM: i32 = 32;
const MARGIN_LEFT: i32 = 32;
const MARGIN_RIGHT: i32 = 32;

const NONE_STRING_OPTION: Option<String> = None;

const TOGGLE_NEIGHBOURS_TEXT_TOGGLED: &str = "Hide Neighbours";
const TOGGLE_NEIGHBOURS_TEXT: &str = "Show Neighbours";

const SCALE_STEP: f64 = 1.0;

#[derive(Debug, Clone)]
struct AnnotationZStack {
    images: Vec<AnnotationImage>,
    best_index: Option<i32>,
}

impl AnnotationZStack {
    pub fn new() -> Self {
        AnnotationZStack {
            images: Vec::<AnnotationImage>::new(),
            best_index: None,
        }
    }
    pub fn push(&mut self, image: AnnotationImage) -> &mut Self {
        self.images.push(image);
        self
    }
    pub fn first(self) -> Option<AnnotationImage> {
        self.images.first().map(|x| x.clone())
    }
}

#[derive(Debug, Clone)]
struct AnnotationImage {
    image_path: String,
    neighbours: [Option<String>; 8],
}

impl AnnotationImage {
    pub fn from_vec(image_path: String, neighbours: Vec<Option<String>>) -> Self {
        let mut _neighbours = [NONE_STRING_OPTION; 8];
        for (index, element) in (0..8).zip(neighbours.iter()) {
            _neighbours[index] = element.clone();
        }

        AnnotationImage {
            image_path,
            neighbours: _neighbours,
        }
    }
}



#[derive(Debug, Clone)]
struct ImageUI {
    individual: std::sync::Arc<Image>,
    center: std::sync::Arc<Image>,
    neighbours: [std::sync::Arc<Image>; 8],
}

impl ImageUI {
    pub fn new() -> ImageUI {
        let individual = std::sync::Arc::new(
            Image::builder()
                .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                .vexpand(true)
                .hexpand(true)
                .build(),
        );
        let center = std::sync::Arc::new(
            Image::builder()
                .vexpand(true)
                .hexpand(true)
                .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                .build(),
        );
        let neighbours = [
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
            std::sync::Arc::new(
                Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .file("/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg")
                    .build(),
            ),
        ];

        ImageUI {
            individual,
            center,
            neighbours,
        }
    }
    pub fn update_image(&self, annotation_image: &AnnotationImage) {
        self.individual
            .set_from_file(Some(annotation_image.image_path.clone()));
        self.center
            .set_from_file(Some(annotation_image.image_path.clone()));

        for index in 0..annotation_image.neighbours.len() {
            self.neighbours[index].set_from_file(annotation_image.neighbours[index].clone());
        }
    }
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        let mut z_stack = AnnotationZStack::new();

        let path = "/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg";
        z_stack.push(AnnotationImage::from_vec(
            path.to_string(),
            vec![
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
            ],
        ));

        let path = "/var/home/hannes/Downloads/test/I12985_X022_Y029_Z5195.jpg";
        z_stack.push(AnnotationImage::from_vec(
            path.to_string(),
            vec![
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
                Some(path.to_string()),
            ],
        ));

        //////////////////
        // MAIN CONTENT //
        //////////////////

        let image_ui = std::sync::Arc::new(ImageUI::new());

        image_ui.update_image(&z_stack.first().unwrap());

        let focus_neighbours_grid = std::sync::Arc::new(
            Grid::builder()
                .vexpand(true)
                .hexpand(true)
                .column_spacing(0)
                .row_spacing(0)
                .build(),
        );

        let focus_neighbours_aspect_frame = AspectFrame::builder()
            .ratio(1.0)
            .xalign(0.5)
            .yalign(0.5)
            .build();
        focus_neighbours_aspect_frame.set_child(Some(image_ui.individual.as_ref()));

        focus_neighbours_grid.attach(image_ui.center.as_ref(), 1, 1, 1, 1);

        for index in 0..image_ui.neighbours.len() {
            // offset index for later images to leave out middle of the grid
            let grid_index: i32 = if index > 3 { index + 1 } else { index }
                .try_into()
                .unwrap();
            let column = grid_index % 3;
            let row = grid_index / 3;
            focus_neighbours_grid.attach(image_ui.neighbours[index].as_ref(), column, row, 1, 1);
            eprintln!("{column} {row}");
        }

        let focus_scale_adjustment = Adjustment::builder()
            .lower(0.0)
            .upper(10.0)
            .value(5.0)
            .step_increment(SCALE_STEP)
            .build();

        let focus_scale = std::sync::Arc::new(
            Scale::builder()
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
                .build(),
        );

        let center_content_seperator = Separator::new(Orientation::Vertical);
        let center_content = Box::builder()
            //.hexpand(true)
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        center_content.append(focus_scale.as_ref());
        center_content.append(&center_content_seperator);
        center_content.append(&focus_neighbours_aspect_frame);

        let focus_image = image_ui.individual.clone();
        focus_scale.connect_value_changed(move |x| {
            eprintln!("Changed value! {:?}", x.value());
            let path = if x.value() > 6.0 {
                "/var/home/hannes/Downloads/test/I12982_X022_Y029_Z5048.jpg"
            } else if x.value() > 3.0 {
                "/var/home/hannes/Downloads/test/I12984_X022_Y029_Z5146.jpg"
            } else {
                "/var/home/hannes/Downloads/test/I12985_X022_Y029_Z5195.jpg"
            };
            focus_image.as_ref().set_from_file(Some(path));
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

        let neighbour_toggle_button = ToggleButton::builder()
            .label(TOGGLE_NEIGHBOURS_TEXT)
            .width_request(158)
            .build();

        let focus_image = image_ui.individual.clone();
        neighbour_toggle_button.connect_toggled(
            clone!(@strong focus_neighbours_grid => move |x| match x.is_active() {
            true => {
                    focus_neighbours_aspect_frame.set_child(Some(focus_neighbours_grid.as_ref()));
                x.set_label(TOGGLE_NEIGHBOURS_TEXT_TOGGLED);
            }
            false => {
                    focus_neighbours_aspect_frame.set_child(Some(focus_image.as_ref()));
                x.set_label(TOGGLE_NEIGHBOURS_TEXT);
            }
            }),
        );
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

        ////////////////////////
        // Keyboard Shortcuts //
        ////////////////////////

        let action_toggle_neighbour = SimpleAction::new("toggle_neighbour", None);
        action_toggle_neighbour.connect_activate(clone!(@weak window => move |_, _| {
            neighbour_toggle_button.set_active(!neighbour_toggle_button.is_active());
        }));

        let action_focus_scale_increment = SimpleAction::new("increment_focus_scale", None);
        action_focus_scale_increment.connect_activate(clone!(@strong focus_scale => move |_, _| {
            focus_scale.set_value(focus_scale.value() + SCALE_STEP);
        }));

        let action_focus_scale_decrement = SimpleAction::new("decrement_focus_scale", None);
        action_focus_scale_decrement.connect_activate(clone!(@strong focus_scale => move |_, _| {
            focus_scale.set_value(focus_scale.value() - SCALE_STEP);
        }));

        let mark_focus = SimpleAction::new("mark_focus", None);
        mark_focus.connect_activate(|_, _| {
            // TODO: implement mark_focus
            eprintln! {"Focus Set!"};
        });

        let skip_focus = SimpleAction::new("skip_focus", None);
        skip_focus.connect_activate(|_, _| {
            // TODO: implement skip focus
            eprintln! {"Skip!"};
        });

        window.add_action(&action_toggle_neighbour);
        window.add_action(&action_focus_scale_increment);
        window.add_action(&action_focus_scale_decrement);
        window.add_action(&mark_focus);
        window.add_action(&skip_focus);

        window.show();
    });

    application.set_accels_for_action("win.toggle_neighbour", &["G"]);
    application.set_accels_for_action("win.increment_focus_scale", &["W"]);
    application.set_accels_for_action("win.decrement_focus_scale", &["S"]);
    application.set_accels_for_action("win.mark_focus", &["M"]);
    application.set_accels_for_action("win.skip_focus", &["N"]);

    application.run();
}
