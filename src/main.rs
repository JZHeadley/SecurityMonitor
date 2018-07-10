extern crate inotify;


use inotify::{
    EventMask,
    Inotify,
    WatchMask,
};
use std::path::PathBuf;


fn main() {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    const FILE_OR_DIR_PATH: &str = "/home/ytpillai/test_rust";
    let current_dir = PathBuf::from(FILE_OR_DIR_PATH);

    inotify
        .add_watch(
            current_dir,
            WatchMask::ALL_EVENTS,
        )
        .expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            println!("{:?}", event.mask);
            if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory modified: {:?}", event.name);
                } else {
                    println!("File modified: {:?} {:?}", event.name, event.wd);
                    let absolute_file_path;
                    match event.name {
                        Some(name) => {
                            let file_name_str = name.to_str(); //OsStr maynot be converted directly to str

                            match file_name_str {
                                Some(nameStr) => absolute_file_path = format!("{}/{}", FILE_OR_DIR_PATH, nameStr),
                                None => absolute_file_path = format!("Could not get filename in {} because it is not convertable to a normal string.", FILE_OR_DIR_PATH),
                            }
                        }
                        None => {
                            absolute_file_path = FILE_OR_DIR_PATH.to_string();
                        }
                    }

                    println!("{:?}", absolute_file_path);
                }
            } else if event.mask.contains(EventMask::CLOSE_NOWRITE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory nowrite: {:?}", event.name);
                } else {
                    println!("File nowrite: {:?} {:?}", event.name, event.wd);
                    let absolute_file_path;
                    match event.name {
                        Some(name) => {
                            let file_name_str = name.to_str(); //OsStr maynot be converted directly to str

                            match file_name_str {
                                Some(nameStr) => absolute_file_path = format!("{}/{}", FILE_OR_DIR_PATH, nameStr),
                                None => absolute_file_path = format!("Could not get filename in {} because it is not convertable to a normal string.", FILE_OR_DIR_PATH)
                            }
                        }
                        None => {
                            absolute_file_path = FILE_OR_DIR_PATH.to_string();
                        }
                    }

                    println!("{:?}", absolute_file_path);
                }
            }
        }
    }
}