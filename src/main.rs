#[macro_use]
extern crate derive_builder;

mod constants;
mod state;
mod ui;

pub use crate::constants::MARGIN_BOTTOM;
pub use crate::state::AnnotationImage;
pub use crate::ui::ImageUI;

use adw::{prelude::*, Application};
use gtk::gio::SimpleAction;
use gtk::glib::{MainContext, PRIORITY_DEFAULT};

use state::{Message, State, UIMessage};

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
    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let mut state = State::new();
    let image_ui = ImageUI::new(app, sender.clone());

    //////////////////
    // MAIN CONTENT //
    //////////////////

    //image_ui.build_ui();

    let _sender = sender.clone();
    image_ui.focus_scale.connect_value_changed(move |x| {
        let index = x.value() as usize;
        _sender.send(Message::FocusLevelChange(index)).unwrap();
    });

    ////////////////////
    // BOTTOM TOOLBAR //
    ///////////////////
    image_ui.back_button.connect_clicked(|button| {
        button
            .activate_action("win.back_focus", None)
            .expect("The action does not exist.");
    });

    image_ui.skip_button.connect_clicked(|button| {
        button
            .activate_action("win.skip_focus", None)
            .expect("The action does not exist.");
    });

    image_ui.focus_button.connect_clicked(|button| {
        button
            .activate_action("win.mark_focus", None)
            .expect("The action does not exist.");
    });

    let _sender = sender.clone();
    image_ui.neighbour_toggle_button.connect_toggled(move |x| {
        _sender
            .send(Message::UI(UIMessage::ShowGrid(x.is_active())))
            .unwrap();
    });

    let _sender = sender.clone();
    image_ui.open_button.connect_clicked(move |_| {
        _sender
            .send(Message::UI(UIMessage::OpenFileChooser))
            .unwrap();
    });

    ////////////////////////
    // Keyboard Shortcuts //
    ////////////////////////
    let _sender = sender.clone();
    let action_toggle_neighbour = SimpleAction::new("toggle_neighbour", None);
    action_toggle_neighbour
        .connect_activate(move |_, _| _sender.send(Message::UI(UIMessage::ToggleGrid)).unwrap());

    let _sender = sender.clone();
    let action_focus_scale_increment = SimpleAction::new("increment_focus_scale", None);
    action_focus_scale_increment.connect_activate(move |_, _| {
        _sender
            .send(Message::UI(UIMessage::IncrementFocus))
            .unwrap()
    });

    let _sender = sender.clone();
    let action_focus_scale_decrement = SimpleAction::new("decrement_focus_scale", None);
    action_focus_scale_decrement.connect_activate(move |_, _| {
        _sender
            .send(Message::UI(UIMessage::DecrementFocus))
            .unwrap()
    });

    let _sender = sender.clone();
    let mark_focus = SimpleAction::new("mark_focus", None);
    mark_focus.connect_activate(move |_, _| {
        _sender.send(Message::MarkFocus).unwrap();
    });

    let _sender = sender.clone();
    let skip_focus = SimpleAction::new("skip_focus", None);
    skip_focus.connect_activate(move |_, _| {
        _sender.send(Message::NextImage).unwrap();
    });

    let _sender = sender.clone();
    let back_focus = SimpleAction::new("back_focus", None);
    back_focus.connect_activate(move |_, _| {
        _sender.send(Message::PreviousImage).unwrap();
    });

    image_ui.window.add_action(&action_toggle_neighbour);
    image_ui.window.add_action(&action_focus_scale_increment);
    image_ui.window.add_action(&action_focus_scale_decrement);
    image_ui.window.add_action(&mark_focus);
    image_ui.window.add_action(&skip_focus);
    image_ui.window.add_action(&back_focus);

    image_ui.show();
    receiver.attach(None, move |msg| {
        eprintln!("Received message: {:?}", msg);
        state.update(&msg);
        image_ui.refresh(&msg, &state);
        Continue(true)
    });
}
