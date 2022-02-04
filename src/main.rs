#[macro_use]
extern crate derive_builder;

mod state;
mod ui;
mod constants;

pub use crate::state::AnnotationImage;
pub use crate::constants::MARGIN_BOTTOM;
use crate::state::AnnotationZStack;
pub use crate::ui::ImageUI;


use std::cell::{RefCell};
use std::fs;
use std::sync::{Arc};

use adw::{prelude::*, Application};
use constants::{TOGGLE_NEIGHBOURS_TEXT_TOGGLED, TOGGLE_NEIGHBOURS_TEXT, SCALE_STEP};
use glib::clone;
use gtk::gio::SimpleAction;
use gtk::{glib, FileChooserDialog, FileChooserAction, ResponseType, FileFilter};

use state::{State};

fn main() {
    let application = Application::builder()
        .application_id("org.kuchelmeister.FocusAnnotator")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_startup(ImageUI::setup_shortcuts);
    application.connect_activate(build_ui);

    application.run();
}


fn build_ui(app: &Application) {

    let state = Arc::new(RefCell::new(State::new()));

    //////////////////
    // MAIN CONTENT //
    //////////////////
    
    let image_ui = Arc::new(ImageUI::new(app));
    //image_ui.build_ui();

    image_ui.focus_scale.connect_value_changed(clone!(@strong image_ui, @strong state => move |x| {
        let index = x.value() as usize;
        state.borrow_mut().set_image_index(Some(index));
        image_ui.update(&state.borrow());
    }));

    ////////////////////
    // BOTTOM TOOLBAR //
    ///////////////////
    image_ui.back_button.connect_clicked(|button| {
        button.activate_action("win.back_focus", None)
        .expect("The action does not exist.");
    });

    image_ui.skip_button.connect_clicked(|button| {
        button.activate_action("win.skip_focus", None)
        .expect("The action does not exist.");
    });

    image_ui.focus_button.connect_clicked(|button| {
        button.activate_action("win.mark_focus", None)
        .expect("The action does not exist.");
    });

    let focus_image = image_ui.individual.clone();
    image_ui.neighbour_toggle_button.connect_toggled(
        clone!(@strong image_ui => move |x| match x.is_active() {
            true => {
                image_ui.focus_neighbours_aspect_frame.set_child(Some(image_ui.focus_neighbours_grid.as_ref()));
                x.set_label(TOGGLE_NEIGHBOURS_TEXT_TOGGLED);
            }
            false => {
                image_ui.focus_neighbours_aspect_frame.set_child(Some(focus_image.as_ref()));
                x.set_label(TOGGLE_NEIGHBOURS_TEXT);
            }
        }),
    );

    image_ui.open_button.connect_clicked(clone!(@strong image_ui, @strong state => move |_| {
            // TODO: actually open and load data
            

            let file_chooser_action = FileChooserAction::Open;
            let buttons = [("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)];
            let filter = FileFilter::new();
            filter.add_pattern(r"*.json");
        
            let file_chooser = Arc::new(FileChooserDialog::new(Some("Chose a data file!"), Some(image_ui.window.as_ref()), file_chooser_action, &buttons));
            file_chooser.set_select_multiple(false);
            file_chooser.set_filter(&filter);
        
            file_chooser.connect_response(clone!(@strong image_ui, @weak state => move |dialog: &FileChooserDialog, response: ResponseType| {
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
    action_toggle_neighbour.connect_activate(clone!(@strong image_ui => move |_, _| {
        image_ui.neighbour_toggle_button.set_active(!image_ui.neighbour_toggle_button.is_active());
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

    image_ui.window.add_action(&action_toggle_neighbour);
    image_ui.window.add_action(&action_focus_scale_increment);
    image_ui.window.add_action(&action_focus_scale_decrement);
    image_ui.window.add_action(&mark_focus);
    image_ui.window.add_action(&skip_focus);
    image_ui.window.add_action(&back_focus);
    image_ui.show();
}