use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, time::Instant};

use gtk::{gio::File, prelude::FileExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::constants::{ANNOTATION_CACHE_FILE_ENDING, NONE_STRING_OPTION};

#[derive(Debug)]
pub enum Message {
    FocusLevelChange(usize),
    MarkFocus,
    NextImage,
    PreviousImage,
    UI(UIMessage),
    OpenFile(File),
    Quit,
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
    annotation_cache: Vec<LightAnnotation>,
    pub root_path: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightAnnotation {
    stack_index: usize,
    focus_image_index: usize,
}

impl LightAnnotation {
    pub fn new(stack_index: usize, focus_image_index: usize) -> Self {
        LightAnnotation {
            stack_index,
            focus_image_index,
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            stacks: Vec::new(),
            stack_index: None,
            focus_image_index: None,
            file_name: None,
            annotation_cache: Vec::new(),
            root_path: None,
        }
    }

    pub fn get_focus_image_index(&self) -> Option<usize> {
        return self.focus_image_index;
    }

    pub fn update(&mut self, msg: &Message) {
        match msg {
            Message::OpenFile(file) => {
                if self.get_file_path() != None {
                    // Save before opening a new file
                    self.save();
                    self.delete_tmp_file();
                }
                self.open(file);
                self.integrate_tmp_file();
                self.delete_tmp_file();
            }
            Message::NextImage => {
                self.skip();
            }
            Message::PreviousImage => {
                self.previous();
            }
            Message::MarkFocus => {
                self.mark_focus();
                self.save_tmp();
                self.skip();
            }
            Message::Quit => {
                self.save();
                self.delete_tmp_file();
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
                let best_index = self.focus_image_index;
                self.stacks[stack_index].best_index = best_index;
                if let Some(best_index) = best_index {
                    self.annotation_cache
                        .push(LightAnnotation::new(stack_index, best_index))
                }
            }
            (_, _) => {}
        }
    }
    pub fn integrate_tmp_file(&mut self) {
        self.get_file_path().map(|mut path| {
            path.set_extension(ANNOTATION_CACHE_FILE_ENDING);

            if path.exists() {
                let contents =
                    fs::read_to_string(path).expect("Something went wrong reading the file");

                self.annotation_cache = serde_json::from_str(&contents).unwrap();
                self.integrate_annotation_cache();
            } else {
                eprintln!("Tmp annotation file {:?} does not exist", path);
            }
        });
    }
    fn integrate_annotation_cache(&mut self) {
        self.annotation_cache.iter().for_each(|annotation| {
            self.stacks.get_mut(annotation.stack_index).map(|x| {
                x.best_index = Some(annotation.focus_image_index);
            });
        });
    }

    pub fn delete_tmp_file(&mut self) {
        self.get_file_path().map(|mut path| {
            path.set_extension(ANNOTATION_CACHE_FILE_ENDING);
            if path.exists() {
                fs::remove_file(path).unwrap();
            }
        });
    }

    pub fn open(&mut self, file: &File) {
        let filename = file.path().expect("Couldn't get file path");
        let now = Instant::now();
        let contents =
            fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");
        let elapsed = now.elapsed();
        println!("Loading file: {:.2?}", elapsed);

        let now = Instant::now();
        let new_dataset: Vec<AnnotationZStack> = serde_json::from_str(&contents).unwrap();
        let elapsed = now.elapsed();
        println!("Deserialisation file: {:.2?}", elapsed);

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

    fn save_file<T: Serialize>(path: PathBuf, content: &T) {
        match fs::File::create(path) {
            Ok(mut file) => {
                let now = Instant::now();
                let contents = serde_json::to_string(content).expect("Could not serialize.");
                let elapsed = now.elapsed();
                println!("Serialization: {:.2?}", elapsed);

                let now = Instant::now();
                match file.write(contents.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "an error occured while saving annotation cache: {}",
                            e.to_string()
                        );
                    }
                }
                let elapsed = now.elapsed();
                println!("Writing to file: {:.2?}", elapsed);
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        };
    }
    fn get_file_path(&self) -> Option<PathBuf> {
        match (self.root_path.clone(), self.file_name.clone()) {
            (Some(root_path), Some(file_name)) => {
                Some(Path::new(&root_path).join(Path::new(&file_name)))
            }
            (_, _) => {
                eprintln!("No save path specified");
                None
            }
        }
    }

    pub fn save_tmp(&self) {
        self.get_file_path().map(|mut path| {
            path.set_extension(ANNOTATION_CACHE_FILE_ENDING);
            State::save_file(path, &self.annotation_cache);
        });
    }

    pub fn save(&self) {
        self.get_file_path().map(|path| {
            State::save_file(path, &self.stacks);
        });
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
