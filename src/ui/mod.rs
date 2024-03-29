use std::{path::Path, sync::Arc};

use adw::{Application, ApplicationWindow, HeaderBar, SplitButton};
use gtk::{
    glib::Sender,
    traits::{
        BoxExt, ButtonExt, DialogExt, FileChooserExt, GridExt, GtkApplicationExt, GtkWindowExt,
        RangeExt, ScaleExt, ToggleButtonExt, WidgetExt,
    },
    ActionBar, AspectFrame, Box, Button, CheckButton, FileChooserAction, FileChooserDialog,
    FileFilter, Grid, Image, Orientation, PositionType, ResponseType, Scale, Separator,
    ToggleButton,
};

use crate::{
    constants::{
        MARGIN_LEFT, MARGIN_RIGHT_SCALE_ADDITIONAL, MARGIN_TOP, SCALE_STEP, TOGGLE_NEIGHBOURS_TEXT,
        TOGGLE_NEIGHBOURS_TEXT_TOGGLED,
    },
    state::{Message, State, UIMessage},
    AnnotationImage, MARGIN_BOTTOM,
};

#[derive(Debug, Clone, Builder)]
pub struct ImageUI {
    pub window: Arc<ApplicationWindow>,
    pub application_vertical_widget: Arc<Box>,
    pub individual: Arc<Image>,
    pub center: Arc<Image>,
    pub neighbours: [Arc<Image>; 8],
    pub focus_scale: Arc<Scale>,
    pub focus_neighbours_grid: Arc<Grid>,
    pub focus_neighbours_aspect_frame: Arc<AspectFrame>,

    pub neighbour_toggle_button: ToggleButton,
    pub skip_marked_checkbox: CheckButton,
    pub open_button: Arc<SplitButton>,
    pub back_button: Arc<Button>,
    pub skip_button: Arc<Button>,
    pub focus_button: Arc<Button>,
    pub sender: Sender<Message>,
}

impl ImageUI {
    pub fn new(app: &Application, sender: Sender<Message>) -> ImageUI {
        let mut builder = ImageUIBuilder::default();
        let application_vertical_widget = Arc::new(Box::new(Orientation::Vertical, 0));

        let window = Arc::new(
            ApplicationWindow::builder()
                .application(app)
                .default_width(800)
                .default_height(600)
                // add content to window
                .content(application_vertical_widget.as_ref())
                .build(),
        );

        builder
            .application_vertical_widget(application_vertical_widget.clone())
            .window(window)
            .sender(sender);
        // TODO: move into builder
        ImageUI::build_header(&mut builder, application_vertical_widget.clone());
        ImageUI::build_center(&mut builder, application_vertical_widget.clone());
        ImageUI::build_bottom_toolbar(&mut builder, application_vertical_widget.clone());

        builder.build().unwrap()
    }

    fn build_header(builder: &mut ImageUIBuilder, application_vertical_widget: Arc<Box>) {
        let header_bar = HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("Focus Annotator", ""))
            .build();

        // TODO: add button functionality
        let open_button = Arc::new(SplitButton::builder().label("Open").build());
        header_bar.pack_start(open_button.as_ref());
        application_vertical_widget.append(&header_bar);

        builder.open_button(open_button);
    }

    fn build_center(builder: &mut ImageUIBuilder, application_vertical_widget: Arc<Box>) {
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

        let focus_neighbours_grid = Arc::new(
            Grid::builder()
                .vexpand(true)
                .hexpand(true)
                .column_spacing(0)
                .row_spacing(0)
                .build(),
        );

        focus_neighbours_grid.attach(center.as_ref(), 1, 1, 1, 1);

        for index in 0..neighbours.len() {
            // offset index for later images to leave out middle of the grid
            let grid_index: i32 = if index > 3 { index + 1 } else { index }
                .try_into()
                .unwrap();
            let column = grid_index % 3;
            let row = grid_index / 3;
            focus_neighbours_grid.attach(neighbours[index].as_ref(), column, row, 1, 1);
            eprintln!("{column} {row}");
        }

        let center_content_seperator = Separator::new(Orientation::Vertical);
        let center_content = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        let focus_neighbours_aspect_frame = Arc::new(
            AspectFrame::builder()
                .ratio(1.0)
                .xalign(0.5)
                .yalign(0.5)
                .build(),
        );
        focus_neighbours_aspect_frame.set_child(Some(individual.as_ref()));

        center_content.append(focus_scale.as_ref());
        center_content.append(&center_content_seperator);
        center_content.append(focus_neighbours_aspect_frame.as_ref());

        application_vertical_widget.append(&center_content);

        builder
            .focus_scale(focus_scale)
            .focus_neighbours_grid(focus_neighbours_grid)
            .focus_neighbours_aspect_frame(focus_neighbours_aspect_frame)
            .individual(individual)
            .center(center)
            .neighbours(neighbours);
    }

    fn build_bottom_toolbar(builder: &mut ImageUIBuilder, application_vertical_widget: Arc<Box>) {
        let bottom_toolbar = ActionBar::builder().build();

        let back_button = Arc::new(Button::builder().label("Back").build());

        let skip_button = Arc::new(Button::builder().label("Skip").build());

        let focus_button = Arc::new(
            Button::builder()
                .label("Set Focus")
                .css_classes(vec!["suggested-action".to_string()])
                .build(),
        );

        let neighbour_toggle_button = ToggleButton::builder()
            .label(TOGGLE_NEIGHBOURS_TEXT)
            .width_request(158)
            .build();

        let skip_marked_checkbox = CheckButton::builder().label("skip marked").build();
        skip_marked_checkbox.activate();

        let focus_skip_link_widget = Box::builder()
            .css_classes(vec!["linked".to_string()])
            .build();
        focus_skip_link_widget.append(back_button.as_ref());
        focus_skip_link_widget.append(skip_button.as_ref());
        focus_skip_link_widget.append(focus_button.as_ref());

        bottom_toolbar.pack_start(&neighbour_toggle_button);

        bottom_toolbar.pack_end(&focus_skip_link_widget);
        bottom_toolbar.pack_end(&skip_marked_checkbox);

        application_vertical_widget.append(&bottom_toolbar);

        builder
            .neighbour_toggle_button(neighbour_toggle_button)
            .skip_marked_checkbox(skip_marked_checkbox)
            .back_button(back_button)
            .skip_button(skip_button)
            .focus_button(focus_button);
    }

    pub fn show(&self) {
        self.window.show();
    }

    pub fn refresh(&self, msg: &Message, state: &State) {
        match msg {
            Message::UI(UIMessage::ToggleGrid) => {
                let curent_state = self.neighbour_toggle_button.is_active();
                self.neighbour_toggle_button.set_active(!curent_state)
            }
            Message::UI(UIMessage::IncrementFocus) => {
                self.focus_scale
                    .set_value(self.focus_scale.value() + SCALE_STEP);
            }
            Message::UI(UIMessage::DecrementFocus) => {
                self.focus_scale
                    .set_value(self.focus_scale.value() - SCALE_STEP);
            }
            Message::UI(UIMessage::ShowGrid(true)) => {
                self.focus_neighbours_aspect_frame
                    .set_child(Some(self.focus_neighbours_grid.as_ref()));
                self.neighbour_toggle_button
                    .set_label(TOGGLE_NEIGHBOURS_TEXT_TOGGLED);
            }
            Message::UI(UIMessage::ShowGrid(false)) => {
                self.focus_neighbours_aspect_frame
                    .set_child(Some(self.individual.as_ref()));
                self.neighbour_toggle_button
                    .set_label(TOGGLE_NEIGHBOURS_TEXT);
            }
            Message::UI(UIMessage::OpenFileChooser) => {
                let file_chooser_action = FileChooserAction::Open;
                let buttons = [("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)];
                let filter = FileFilter::new();
                filter.add_pattern(r"*.json");

                let file_chooser = FileChooserDialog::new(
                    Some("Chose a data file!"),
                    Some(self.window.as_ref()),
                    file_chooser_action,
                    &buttons,
                );
                file_chooser.set_select_multiple(false);
                file_chooser.set_filter(&filter);

                let _sender = self.sender.clone();
                file_chooser.connect_response(
                    move |dialog: &FileChooserDialog, response: ResponseType| {
                        if response == ResponseType::Ok {
                            let file = dialog.file().expect("Couldn't get file");
                            eprintln!("Open");
                            _sender.send(Message::OpenFile(file)).unwrap();
                        }
                        dialog.close();
                    },
                );

                file_chooser.show();
            }
            Message::UI(UIMessage::RefreshImages)
            | Message::NextImage
            | Message::PreviousImage
            | Message::MarkFocus
            | Message::FocusLevelChange(_)
            | Message::OpenFile(_) => {
                match (
                    state.get_current_annotation_image(),
                    state.root_path.clone(),
                ) {
                    (Some(annotation_image), Some(base_path)) => {
                        self.update_image(&annotation_image, base_path)
                    }
                    (_, _) => {
                        // TODO: write error message
                    }
                }
                self.update_focus_scale(&state);
            }
            Message::Quit | Message::SkipMarkedToogled(_) => {}
        }
    }
    fn update_image(&self, annotation_image: &AnnotationImage, base_path: String) {
        self.individual.set_from_file(Some(
            Path::new(&base_path).join(Path::new(&annotation_image.image_path)),
        ));
        self.center.set_from_file(Some(
            Path::new(&base_path).join(Path::new(&annotation_image.image_path)),
        ));

        for index in 0..annotation_image.neighbours.len() {
            self.neighbours[index].set_from_file(
                annotation_image.neighbours[index]
                    .clone()
                    .map(|x| Path::new(&base_path).join(Path::new(&x))),
            );
        }
    }

    fn update_focus_scale(&self, state: &State) {
        let max = state.get_current_foucs_stack_max().unwrap_or(0) as f64;
        self.focus_scale.set_range(0.0, max);

        if let Some(best_index) = state.get_current_foucs_stack_best_index() {
            self.focus_scale.clear_marks();
            self.focus_scale
                .add_mark(best_index as f64, PositionType::Right, Some("focus"));
            self.focus_scale.set_margin_end(0);
        } else {
            self.focus_scale.clear_marks();
            self.focus_scale
                .set_margin_end(MARGIN_RIGHT_SCALE_ADDITIONAL);
        }

        if let Some(current_value) = state.get_focus_image_index() {
            self.focus_scale.set_value(current_value as f64);
        } else {
            self.focus_scale.set_value(f64::floor(max / 2.0));
        }
    }

    pub fn setup_shortcuts(app: &Application) {
        app.set_accels_for_action("win.toggle_neighbour", &["G"]);
        app.set_accels_for_action("win.increment_focus_scale", &["W"]);
        app.set_accels_for_action("win.decrement_focus_scale", &["S"]);
        app.set_accels_for_action("win.mark_focus", &["M"]);
        app.set_accels_for_action("win.skip_focus", &["N"]);
        app.set_accels_for_action("win.back_focus", &["B"]);
    }
}
