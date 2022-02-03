use serde::{Serialize, Deserialize};

use crate::constants::NONE_STRING_OPTION;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnnotationZStack {
        pub images: Vec<AnnotationImage>,
        pub best_index: Option<usize>,
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