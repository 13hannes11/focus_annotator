use serde::{Serialize, Deserialize};

use crate::constants::NONE_STRING_OPTION;
#[derive(Debug, Clone)]
pub struct State{
        stacks: Vec<AnnotationZStack>,
        stack_index: Option<usize>,
        pub image_index: Option<usize>,
    }

    impl State {
        pub fn new() -> Self {
            State{
                stacks: Vec::new(),
                stack_index: None,
                image_index: None,
            }
        }
        pub fn set_image_index(&mut self, image_index : Option<usize>) {
            self.image_index = image_index;
        }

        pub fn get_current_annotation_image(&self) -> Option<AnnotationImage>{
            match self.image_index {
                Some(image_index) => {
                    let stack = self.get_current_focus_stack();
                    match stack {
                        Some(stack) => {
                            stack.images.get(image_index).map(|x| x.clone())
                        },
                        _ => None
                    }
                }                
                _ => None
            }
        }

        pub fn get_current_focus_stack(&self) -> Option<&AnnotationZStack> {
            match self.stack_index {
                Some(stack_index) => {
                    self.stacks.get(stack_index)
                }
                _ => None,
            }
        }

        pub fn replace_foucs_stacks(&mut self, mut stacks : Vec<AnnotationZStack>){
            self.stacks.clear();
            self.stacks.append(&mut stacks);
            eprintln!("{}",  stacks.len());

            if let Some(z_stack) = self.stacks.first() {
                self.stack_index = Some(0);
                self.image_index = if let Some(_) = z_stack.images.first() {
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
                _ => None
            }
        }

        pub fn skip(&mut self) {
            let len = self.stacks.len();
            if len == 0 {
                self.stack_index = None;
            } else if self.stack_index.map_or_else(|| false, |x| {x + 1 < len}) {
                self.stack_index = self.stack_index.map(|x| x+1)
            }

            eprintln!("{:?}", self.stack_index)
        }

        pub fn mark_focus(&mut self) {
            match (self.stack_index, self.image_index) {
                (Some(stack_index), Some(_)) => {
                    self.stacks[stack_index].best_index = self.image_index;
                }
                (_, _) => {}
            }
        }

        pub fn previous(&mut self) {
            let len = self.stacks.len();
            if len == 0 {
                self.stack_index = None;
            } else if self.stack_index.map_or_else(|| false, |x| x > 0) {
                self.stack_index = self.stack_index.map(|x| x-1)
            }
        }
    
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnnotationZStack {
        pub images: Vec<AnnotationImage>,
        pub best_index: Option<usize>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnnotationImage {
        pub image_path: String,
        pub neighbours: [Option<String>; 8],
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