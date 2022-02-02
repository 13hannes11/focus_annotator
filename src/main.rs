use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::fs;

use serde::{Deserialize, Serialize};

use adw::{prelude::*, ApplicationWindow, HeaderBar, SplitButton};
use gio::SimpleAction;
use glib::clone;
use gtk::{gio, glib, FileChooserAction, FileChooserDialog, ResponseType};
use gtk::{
    ActionBar, Application, AspectFrame, Box, Button, FileFilter, Grid, Image, Orientation,
    PositionType, Scale, Separator, ToggleButton,
};

const MARGIN_TOP: i32 = 32;
const MARGIN_BOTTOM: i32 = 32;
const MARGIN_LEFT: i32 = 16;
const MARGIN_RIGHT_SCALE_ADDITIONAL: i32 = 38;

const NONE_STRING_OPTION: Option<String> = None;

const TOGGLE_NEIGHBOURS_TEXT_TOGGLED: &str = "Hide Neighbours";
const TOGGLE_NEIGHBOURS_TEXT: &str = "Show Neighbours";

const SCALE_STEP: f64 = 1.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnnotationZStack {
    images: Vec<AnnotationImage>,
    best_index: Option<usize>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    individual: Arc<Image>,
    center: Arc<Image>,
    neighbours: [Arc<Image>; 8],
}

impl ImageUI {
    pub fn new() -> ImageUI {
        let individual = Arc::new(Image::builder().vexpand(true).hexpand(true).build());
        let center = Arc::new(Image::builder().vexpand(true).hexpand(true).build());
        let neighbours = [
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
            Arc::new(Image::builder().vexpand(true).hexpand(true).build()),
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

fn update_focus_scale(focus_scale: &Scale, z_stack: AnnotationZStack) {
    let max = (z_stack.images.len() - 1) as f64;
    focus_scale.set_range(0.0, max);
    focus_scale.set_value(f64::floor(max / 2.0));

    if z_stack.best_index.is_some() {
        focus_scale.add_mark(
            z_stack.best_index.unwrap() as f64,
            PositionType::Right,
            Some("focus"),
        );
        focus_scale.set_margin_end(0);
    } else {
        focus_scale.clear_marks();
        focus_scale.set_margin_end(MARGIN_RIGHT_SCALE_ADDITIONAL);
    }
}

fn change_image(
    direction: i32,
    current_z_stack_index: &Cell<usize>,
    annotaion_dataset: Vec<AnnotationZStack>,
    focus_scale: &Scale,
    image_ui: &ImageUI,
) {
    let index = current_z_stack_index.get() as i32 + direction;

    eprintln!("Index after {index}");
    // Makes sure we are not overstepping bounds
    let index = if index < annotaion_dataset.len() as i32 && index >= 0 {
        current_z_stack_index.set(index.try_into().unwrap_or(0));
        index as usize
    } else {
        current_z_stack_index.get()
    };
    eprintln!("Index after {index}");

    let z_stack = annotaion_dataset[index].clone();
    update_focus_scale(&focus_scale, z_stack);

    let img = annotaion_dataset[index].images[focus_scale.value() as usize].clone();
    image_ui.update_image(&img);
}

fn next_image(
    current_z_stack_index: &Cell<usize>,
    annotaion_dataset: Vec<AnnotationZStack>,
    focus_scale: &Scale,
    image_ui: &ImageUI,
) {
    change_image(
        1,
        current_z_stack_index,
        annotaion_dataset,
        focus_scale,
        image_ui,
    );
}

fn previous_image(
    current_z_stack_index: &Cell<usize>,
    annotaion_dataset: Vec<AnnotationZStack>,
    focus_scale: &Scale,
    image_ui: &ImageUI,
) {
    change_image(
        -1,
        current_z_stack_index,
        annotaion_dataset,
        focus_scale,
        image_ui,
    );
}

fn save_annotation(annotation_dataset: &Vec<AnnotationZStack>) {
    // TODO: implement saving
    eprintln!("Saving is not implemented yet!");
    // Serialize it to a JSON string.
    let j = serde_json::to_string(&annotation_dataset).unwrap();

    // Print, write to a file, or send to an HTTP server.
    eprintln!("{}", &j);
}

fn main() {
    let application = Application::builder()
        .application_id("org.kuchelmeister.FocusAnnotator")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_startup(setup_shortcuts);
    application.connect_activate(build_ui);

    application.run();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.toggle_neighbour", &["G"]);
    app.set_accels_for_action("win.increment_focus_scale", &["W"]);
    app.set_accels_for_action("win.decrement_focus_scale", &["S"]);
    app.set_accels_for_action("win.mark_focus", &["M"]);
    app.set_accels_for_action("win.skip_focus", &["N"]);
    app.set_accels_for_action("win.back_focus", &["B"]);

}

fn build_ui(app: &Application) {
    let current_z_stack_index = Arc::new(Cell::new(0));
    let annotaion_dataset = Arc::new(Mutex::new(Vec::<AnnotationZStack>::new()));

    let mut z_stack = AnnotationZStack::new();

    let path = "/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03987/I03987_X008_Y026_Z5498_0_1200.jpg";
    z_stack.push(AnnotationImage::from_vec(
        path.to_string(),
        vec![
            None,
            None,
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03987/I03987_X008_Y026_Z5498_0_1125.jpg".to_string()),
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03987/I03987_X008_Y026_Z5498_75_1125.jpg".to_string()),
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03987/I03987_X008_Y026_Z5498_75_1200.jpg".to_string()),
            None,
            None,
        ],
    ));

    let path = "/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03988/I03988_X008_Y026_Z5566_0_1200.jpg";
    z_stack.push(AnnotationImage::from_vec(
        path.to_string(),
        vec![
            None,
            None,
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03988/I03988_X008_Y026_Z5566_0_1125.jpg".to_string()),
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03988/I03988_X008_Y026_Z5566_75_1125.jpg".to_string()),
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03988/I03988_X008_Y026_Z5566_75_1200.jpg".to_string()),
            None,
            None,
        ],
    ));

    let path = "/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03989/I03989_X008_Y026_Z5703_0_1200.jpg";
    z_stack.push(AnnotationImage::from_vec(
        path.to_string(),
        vec![
            None,
            None,
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03989/I03989_X008_Y026_Z5703_0_1125.jpg".to_string()),
            None,
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03989/I03989_X008_Y026_Z5703_75_1125.jpg".to_string()),
            Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/31/I03989/I03989_X008_Y026_Z5703_75_1200.jpg".to_string()),
            None,
            None,
        ],
    ));

    annotaion_dataset.lock().unwrap().push(z_stack.clone());

    {
        let mut z_stack = AnnotationZStack::new();

        z_stack.best_index = Some(0);

        let path = "/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg";
        z_stack.push(AnnotationImage::from_vec(
            path.to_string(),
            vec![
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),    
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),    
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),                    
                Some("/var/home/hannes/Documents/toolbox/python/thesis/focus_metrics_test/img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg".to_string()),
            ],
        ));

        annotaion_dataset.lock().unwrap().push(z_stack.clone());
    }

    //////////////////
    // MAIN CONTENT //
    //////////////////

    let image_ui = Arc::new(ImageUI::new());

    image_ui
        .as_ref()
        .update_image(&z_stack.clone().first().unwrap());

    let focus_neighbours_grid = Arc::new(
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

    let focus_scale = Arc::new(
        Scale::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .margin_top(MARGIN_TOP)
            .margin_bottom(MARGIN_BOTTOM)
            .margin_start(MARGIN_LEFT)
            .draw_value(true)
            .inverted(true)
            .round_digits(0)
            .digits(0)
            .build(),
    );

    update_focus_scale(focus_scale.as_ref(), z_stack.clone());

    let center_content_seperator = Separator::new(Orientation::Vertical);
    let center_content = Box::builder()
        //.hexpand(true)
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();

    center_content.append(focus_scale.as_ref());
    center_content.append(&center_content_seperator);
    center_content.append(&focus_neighbours_aspect_frame);

    focus_scale.connect_value_changed(clone!(@strong image_ui, @strong z_stack => move |x| {
        let index = x.value() as usize;
        let img = z_stack.images[index].clone();
        image_ui.update_image(&img);
    }));

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

    let back_button = Button::builder().label("Back").build();

    back_button.connect_clicked(|button| {
        button.activate_action("win.back_focus", None)
        .expect("The action does not exist.");
    });

    let skip_button = Button::builder().label("Skip").build();

    skip_button.connect_clicked(|button| {
        button.activate_action("win.skip_focus", None)
        .expect("The action does not exist.");
    });

    let focus_button = Button::builder()
        .label("Set Focus")
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    focus_button.connect_clicked(|button| {
        button.activate_action("win.mark_focus", None)
        .expect("The action does not exist.");
    });
    let focus_skip_link_widget = Box::builder()
        .css_classes(vec!["linked".to_string()])
        .build();
    focus_skip_link_widget.append(&back_button);
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

    let file_chooser_action = FileChooserAction::Open;
    let buttons = [("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)];
    let filter = FileFilter::new();
    filter.add_pattern(r"*.json");

    let file_chooser = Arc::new(FileChooserDialog::new(Some("Chose a data file!"), Some(&window), file_chooser_action, &buttons));
    file_chooser.set_select_multiple(false);
    file_chooser.set_filter(&filter);

    file_chooser.connect_response(clone!(@strong annotaion_dataset => move |dialog: &FileChooserDialog, response: ResponseType| {
        if response == ResponseType::Ok {
            let file = dialog.file().expect("Couldn't get file");
            eprintln!("Open");
            let filename = file.path().expect("Couldn't get file path");
            let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
            
            let mut dataset : Vec<AnnotationZStack> = serde_json::from_str(&contents).unwrap();
            eprintln!("{}", contents);
            //annotaion_dataset.lock().unwrap().clear();
            //annotaion_dataset.lock().unwrap().append(&mut dataset);
            // TODO: update data after loading
        }
        dialog.close();        
    }));

    open_button.connect_clicked(clone!(@weak window, @strong file_chooser => move |_| {
            // TODO: actually open and load data
            file_chooser.show();

    }));

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
    mark_focus.connect_activate(clone!(@strong image_ui, @strong focus_scale, @strong current_z_stack_index, @strong annotaion_dataset => move |_, _| {
        eprintln! {"Focus Set!"};
        let index = current_z_stack_index.as_ref().get();
        annotaion_dataset.lock().unwrap()[index].best_index = Some(focus_scale.value() as usize);

        save_annotation(&annotaion_dataset.lock().unwrap());
        next_image(current_z_stack_index.clone().as_ref(), annotaion_dataset.lock().unwrap().clone(), focus_scale.as_ref(), image_ui.as_ref());
    }));

    let skip_focus = SimpleAction::new("skip_focus", None);
    skip_focus.connect_activate(clone!(@strong image_ui, @strong focus_scale, @strong current_z_stack_index, @strong annotaion_dataset => move |_, _| {
        next_image(current_z_stack_index.clone().as_ref(), annotaion_dataset.lock().unwrap().clone(), focus_scale.as_ref(), image_ui.as_ref());
    }));

    let back_focus = SimpleAction::new("back_focus", None);
    back_focus.connect_activate(clone!(@strong image_ui, @strong focus_scale, @strong current_z_stack_index, @strong annotaion_dataset => move |_, _| {
        previous_image(current_z_stack_index.clone().as_ref(), annotaion_dataset.lock().unwrap().clone(), focus_scale.as_ref(), image_ui.as_ref());
    }));

    window.add_action(&action_toggle_neighbour);
    window.add_action(&action_focus_scale_increment);
    window.add_action(&action_focus_scale_decrement);
    window.add_action(&mark_focus);
    window.add_action(&skip_focus);
    window.add_action(&back_focus);

    window.show();
}