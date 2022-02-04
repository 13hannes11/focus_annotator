mod state;
mod constants;

pub use crate::state::AnnotationImage;
pub use crate::constants::MARGIN_BOTTOM;


use std::cell::{RefCell};
use std::sync::{Arc};
use std::{fs};

use adw::{prelude::*, ApplicationWindow, HeaderBar, SplitButton};
use constants::{MARGIN_TOP, MARGIN_LEFT, MARGIN_RIGHT_SCALE_ADDITIONAL, TOGGLE_NEIGHBOURS_TEXT, TOGGLE_NEIGHBOURS_TEXT_TOGGLED, SCALE_STEP};
use gio::SimpleAction;
use glib::clone;
use gtk::{gio, glib, FileChooserAction, FileChooserDialog, ResponseType};
use gtk::{
    ActionBar, Application, AspectFrame, Box, Button, FileFilter, Grid, Image, Orientation,
    PositionType, Scale, Separator, ToggleButton,
};
use state::{AnnotationZStack, State};

#[derive(Debug, Clone)]
struct ImageUI {
    individual: Arc<Image>,
    center: Arc<Image>,
    neighbours: [Arc<Image>; 8],
    focus_scale: Arc<Scale>,
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

        ImageUI {
            individual,
            center,
            neighbours,
            focus_scale,
        }
    }
    pub fn update(&self, state : &State) {
        if let Some(annotation_image) = state.get_current_annotation_image() {
            self.update_image(&annotation_image);
        }
        self.update_focus_scale(&state);
    }
    fn update_image(&self, annotation_image: &AnnotationImage) {
        self.individual
            .set_from_file(Some(annotation_image.image_path.clone()));
        self.center
            .set_from_file(Some(annotation_image.image_path.clone()));

        for index in 0..annotation_image.neighbours.len() {
            self.neighbours[index].set_from_file(annotation_image.neighbours[index].clone());
        }
    }

    fn update_focus_scale(&self, state: &State) {
        let max = state.get_current_foucs_stack_max().unwrap_or(0) as f64;
        self.focus_scale.set_range(0.0, max);

        if  let Some(best_index) = state.get_current_foucs_stack_best_index() {
            self.focus_scale.clear_marks();
            self.focus_scale.add_mark(
                best_index as f64,
                PositionType::Right,
                Some("focus"),
            );
            self.focus_scale.set_margin_end(0);
        } else {
            self.focus_scale.clear_marks();
            self.focus_scale.set_margin_end(MARGIN_RIGHT_SCALE_ADDITIONAL);
        }

        if let Some(current_value) = state.image_index {
            self.focus_scale.set_value(current_value as f64);
        } else {
            self.focus_scale.set_value(f64::floor(max / 2.0));
        }
    }
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

    let state = Arc::new(RefCell::new(State::new()));

    //////////////////
    // MAIN CONTENT //
    //////////////////
    
    let image_ui = Arc::new(ImageUI::new());
    image_ui.update(&(state.as_ref().borrow()));

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

    //let focus_scale = image_ui.focus_scale.clone();

    // update_focus_scale(focus_scale.as_ref(), z_stack.clone());

    let center_content_seperator = Separator::new(Orientation::Vertical);
    let center_content = Box::builder()
        //.hexpand(true)
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();

    center_content.append(image_ui.focus_scale.as_ref());
    center_content.append(&center_content_seperator);
    center_content.append(&focus_neighbours_aspect_frame);

    image_ui.focus_scale.connect_value_changed(clone!(@strong image_ui, @strong state => move |x| {
        let index = x.value() as usize;
        state.borrow_mut().set_image_index(Some(index));
        image_ui.update(&state.borrow());
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

    open_button.connect_clicked(clone!(@weak window, @strong image_ui, @strong state => move |_| {
            // TODO: actually open and load data
            

            let file_chooser_action = FileChooserAction::Open;
            let buttons = [("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)];
            let filter = FileFilter::new();
            filter.add_pattern(r"*.json");
        
            let file_chooser = Arc::new(FileChooserDialog::new(Some("Chose a data file!"), Some(&window), file_chooser_action, &buttons));
            file_chooser.set_select_multiple(false);
            file_chooser.set_filter(&filter);
        
            file_chooser.connect_response(clone!(@weak window, @strong image_ui, @weak state => move |dialog: &FileChooserDialog, response: ResponseType| {
                if response == ResponseType::Ok {
                    let file = dialog.file().expect("Couldn't get file");
                    eprintln!("Open");
                    let filename = file.path().expect("Couldn't get file path");
                    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
                    eprintln!("{}", contents);

                    let new_dataset : Vec<AnnotationZStack> = serde_json::from_str(&contents).unwrap();
                    let mut state = state.borrow_mut();
                    
                    state.replace_foucs_stacks(new_dataset);
                    image_ui.update(&state);
                }
                dialog.close();        
            }));

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
    action_focus_scale_increment.connect_activate(clone!(@strong image_ui => move |_, _| {
        image_ui.focus_scale.set_value(image_ui.focus_scale.value() + SCALE_STEP);
    }));

    let action_focus_scale_decrement = SimpleAction::new("decrement_focus_scale", None);
    action_focus_scale_decrement.connect_activate(clone!(@strong image_ui => move |_, _| {
        image_ui.focus_scale.set_value(image_ui.focus_scale.value() - SCALE_STEP);
    }));

    let mark_focus = SimpleAction::new("mark_focus", None);
    mark_focus.connect_activate(clone!(@strong image_ui, @strong state => move |_, _| {
        eprintln! {"Focus Set!"};

        let mut state = state.borrow_mut();
        state.mark_focus();
        state.skip();
        image_ui.update(&state);
    }));

    let skip_focus = SimpleAction::new("skip_focus", None);
    skip_focus.connect_activate(clone!(@strong image_ui, @strong state => move |_, _| {
        let mut state = state.borrow_mut();
        state.skip();
        image_ui.update(&state);
    }));

    let back_focus = SimpleAction::new("back_focus", None);
    back_focus.connect_activate(clone!(@strong image_ui, @strong state => move |_, _| {
        let mut state = state.borrow_mut();
        state.previous();
        image_ui.update(&state);
    }));

    window.add_action(&action_toggle_neighbour);
    window.add_action(&action_focus_scale_increment);
    window.add_action(&action_focus_scale_decrement);
    window.add_action(&mark_focus);
    window.add_action(&skip_focus);
    window.add_action(&back_focus);

    window.show();
}