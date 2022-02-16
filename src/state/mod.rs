use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use gtk::{gio::File, prelude::FileExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::constants::NONE_STRING_OPTION;

#[derive(Debug)]
pub enum Message {
    FocusLevelChange(usize),
    MarkFocus,
    NextImage,
    PreviousImage,
    UI(UIMessage),
    OpenFile(File),
}

// Messages that do not impact state
#[derive(Debug)]
pub enum UIMessage {
    OpenFileChooser,
    RefreshImages,
    ToggleGrid,
    DecrementFocus,
    IncrementFocus,
    ShowGrid(bool),
}

#[derive(Debug, Clone)]
pub struct State {
    stacks: Vec<AnnotationZStack>,
    stack_index: Option<usize>,
    focus_image_index: Option<usize>,
    file_name: Option<String>,
    pub root_path: Option<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            stacks: Vec::new(),
            stack_index: None,
            focus_image_index: None,
            file_name: None,
            root_path: None,
        }
    }

    pub fn get_focus_image_index(&self) -> Option<usize> {
        return self.focus_image_index;
    }

    pub fn update(&mut self, msg: &Message) {
        match msg {
            Message::OpenFile(file) => {
                let filename = file.path().expect("Couldn't get file path");
                let contents = fs::read_to_string(filename.clone())
                    .expect("Something went wrong reading the file");
                //eprintln!("{}", contents);

                let new_dataset: Vec<AnnotationZStack> = serde_json::from_str(&contents).unwrap();
                self.replace_foucs_stacks(new_dataset);
                self.file_name = filename.clone().as_path().file_name().map(|x| {
                    x.to_str()
                        .expect("failed to convert filname to str")
                        .to_string()
                });
                self.root_path = filename.clone().as_path().parent().map(|x| {
                    x.to_str()
                        .expect("failed to convert filname to str")
                        .to_string()
                });

                match (self.root_path.clone(), self.file_name.clone()) {
                    (Some(root_path), Some(file_name)) => {
                        let path = Path::new(&root_path).join(Path::new(&file_name));
                        eprintln!("{:?}", path);
                    }
                    (_, _) => {
                        eprintln!("Path not properly set");
                    }
                }
            }
            Message::NextImage => {
                self.skip();
            }
            Message::PreviousImage => {
                self.previous();
            }
            Message::MarkFocus => {
                self.mark_focus();
                self.save();
                self.skip();
            }
            Message::FocusLevelChange(lvl) => {
                self.set_focus_image_index(Some(*lvl));
            }
            Message::UI(_) => {}
        }
    }

    pub fn set_focus_image_index(&mut self, image_index: Option<usize>) {
        self.focus_image_index = image_index;
    }

    pub fn get_current_annotation_image(&self) -> Option<AnnotationImage> {
        match self.focus_image_index {
            Some(image_index) => {
                let stack = self.get_current_focus_stack();
                match stack {
                    Some(stack) => stack.images.get(image_index).map(|x| x.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_current_focus_stack(&self) -> Option<&AnnotationZStack> {
        match self.stack_index {
            Some(stack_index) => self.stacks.get(stack_index),
            _ => None,
        }
    }

    pub fn replace_foucs_stacks(&mut self, mut stacks: Vec<AnnotationZStack>) {
        self.stacks.clear();
        self.stacks.append(&mut stacks);
        eprintln!("{}", stacks.len());

        if let Some(z_stack) = self.stacks.first() {
            self.stack_index = Some(0);
            self.focus_image_index = if let Some(_) = z_stack.images.first() {
                Some(0)
            } else {
                None
            }
        } else {
            self.stack_index = None;
        }
    }
    pub fn get_current_foucs_stack_max(&self) -> Option<usize> {
        self.get_current_focus_stack().map(|x| x.images.len() - 1)
    }
    pub fn get_current_foucs_stack_best_index(&self) -> Option<usize> {
        match self.get_current_focus_stack() {
            Some(stack) => stack.best_index,
            _ => None,
        }
    }

    pub fn skip(&mut self) {
        let len = self.stacks.len();
        if len == 0 {
            self.stack_index = None;
        } else if self.stack_index.map_or_else(|| false, |x| x + 1 < len) {
            self.stack_index = self.stack_index.map(|x| x + 1)
        }

        eprintln!("{:?}", self.stack_index)
    }

    pub fn mark_focus(&mut self) {
        match (self.stack_index, self.focus_image_index) {
            (Some(stack_index), Some(_)) => {
                self.stacks[stack_index].best_index = self.focus_image_index;
            }
            (_, _) => {}
        }
    }

    pub fn save(&self) {
        match (self.root_path.clone(), self.file_name.clone()) {
            (Some(root_path), Some(file_name)) => {
                let path = Path::new(&root_path).join(Path::new(&file_name));
                match fs::File::create(path) {
                    Ok(mut file) => {
                        use std::time::Instant;
                        let now = Instant::now();
                        let contents =
                            serde_json::to_string(&self.stacks).expect("Could not serialize data.");
                        let elapsed = now.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);

                        let now = Instant::now();
                        match file.write(contents.as_bytes()) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("an error occured while saving: {}", e.to_string());
                            }
                        }
                        let elapsed = now.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                    }
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                };
            }
            (_, _) => {
                // TODO: error dialogue
                eprintln!("No save path specified");
            }
        }
    }

    pub fn previous(&mut self) {
        let len = self.stacks.len();
        if len == 0 {
            self.stack_index = None;
        } else if self.stack_index.map_or_else(|| false, |x| x > 0) {
            self.stack_index = self.stack_index.map(|x| x - 1)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationZStack {
    pub images: Vec<AnnotationImage>,
    pub best_index: Option<usize>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationImage {
    pub image_path: String,
    pub neighbours: [Option<String>; 8],

    #[serde(flatten)]
    extra: HashMap<String, Value>,
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
            extra: HashMap::new(),
        }
    }
}
