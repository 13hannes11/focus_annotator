# Focus Annotator

Focus Annotator is a tool for annotation the focal plane of a part of an image. It is a tool I built in rust as part of my master's thesis to make image focus annotations for z-stack images quicker.

![image](https://user-images.githubusercontent.com/9381167/152766149-18ac6194-5586-4434-8a63-70b46496bb37.png)



## Installation

Currently the application can only be optained by compiling from source.

## Compilation Instructions

### Docker

The tool can be compiled with docker-compose:

* Linux Version `docker-compose up` (found in *target/release/*, dependencies are not included therefore install `libadwaita` via your distros package manager)
* Windows Version: `docker-compose -f docker-compose.windows.yml up` (the resulting file is packaged as *package.zip*, dependencies included)


### Native

The prerequisits can be installed as described in the gtk4-rs book:

* [Linux](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)
* [Windows](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html) (untested)
* [Mac OS](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_macos.html) (untested)

After the prerequisits are installed you can run/build the tool as is usual for rust:

```
cargo run
```

## Usage

When opening the program open a .json file that describes where to find the images. The file should look as follows.

```json
[
    {
        "images": [
            {
                "image_path": "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                "neighbours": [
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg",
                    "img/30_753da05d-cd1e-45c5-8593-003323e0bb69_I00243_X013_Y003_Z4648.jpg"
                ]
            }
        ],
        "best_index": 0
    }
    {
        "images": [
            {
                "image_path": "img/31/I03987/I03987_X008_Y026_Z5498_0_1200.jpg",
                "neighbours": [
                    null,
                    null,
                    null,
                    "img/31/I03987/I03987_X008_Y026_Z5498_0_1125.jpg",
                    null,
                    "img/31/I03987/I03987_X008_Y026_Z5498_75_1125.jpg",
                    "img/31/I03987/I03987_X008_Y026_Z5498_75_1200.jpg",
                    null
                ]
            },
            {
                "image_path": "img/31/I03988/I03988_X008_Y026_Z5566_0_1200.jpg",
                "neighbours": [
                    null,
                    null,
                    null,
                    "img/31/I03988/I03988_X008_Y026_Z5566_0_1125.jpg",
                    null,
                    "img/31/I03988/I03988_X008_Y026_Z5566_75_1125.jpg",
                    "img/31/I03988/I03988_X008_Y026_Z5566_75_1200.jpg",
                    null
                ]
            },
            {
                "image_path": "img/31/I03989/I03989_X008_Y026_Z5703_0_1200.jpg",
                "neighbours": [
                    null,
                    null,
                    null,
                    "img/31/I03989/I03989_X008_Y026_Z5703_0_1125.jpg",
                    null,
                    "img/31/I03989/I03989_X008_Y026_Z5703_75_1125.jpg",
                    "img/31/I03989/I03989_X008_Y026_Z5703_75_1200.jpg",
                    null
                ],
            }
        ],
        "best_index": null
    }
]
```

The json file contains a list of unamed objets. Each object corresponds to one focus stack. Best index is set to null for non annotated images and is a number corresponding to the image in focus once annotated. The array `image` present in each focus stack contains objects tat represents the individual images.

Each individual image has an `image_path` (mandatory) which is *relative* to the *position* of the *json file*, and **8** (!) `neighbours` (which are allowed to be set to `null`). Neighbours are represented indexed the following way:

```
0 1 2
3 - 4
5 6 7
```


You are allowed to store additional data in focus stack objects (and image objects) and this should be preserved when using the tool, however, make sure to back up the metadata file before using the tool.

## Keyboard shortcuts

The tool supports keyboard shortcuts:

- `w` - move up in the focus stack
- `s` - move down in the focus stack
- `b` - *back* - go back one image
- `n` - *next* - skip image
- `m` - *mark* - mark current image in the z-stack as in focus and go to next image

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
