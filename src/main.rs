extern crate env_logger;
extern crate fuse;
extern crate libc;
extern crate time;

use fuse::Filesystem;
use std::env;
use std::io::ErrorKind;
use std::path::Path;

struct NullFS;

impl Filesystem for NullFS {}

fn main() {
    testHelloFs();
}

fn testHelloFs() {
    use std::env;
    use std::ffi::OsStr;
    use libc::ENOENT;
    use time::Timespec;
    use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

    const TTL: Timespec = Timespec { sec: 1, nsec: 0 };                     // 1 second

    const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56

    const HELLO_DIR_ATTR: FileAttr = FileAttr {
        ino: 1,
        size: 0,
        blocks: 0,
        atime: CREATE_TIME,
        mtime: CREATE_TIME,
        ctime: CREATE_TIME,
        crtime: CREATE_TIME,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    };

    const HELLO_TXT_CONTENT: &'static str = "Hello World!\n";

    const HELLO_TXT_ATTR: FileAttr = FileAttr {
        ino: 2,
        size: 13,
        blocks: 1,
        atime: CREATE_TIME,
        mtime: CREATE_TIME,
        ctime: CREATE_TIME,
        crtime: CREATE_TIME,
        kind: FileType::RegularFile,
        perm: 0o644,
        nlink: 1,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    };

    struct HelloFS;

    impl Filesystem for HelloFS {
        fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
            if parent == 1 && name.to_str() == Some("hello.txt") {
                println!("WELOOKEDUPDAFILE");
                reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
            } else {
                reply.error(ENOENT);
            }
        }

        fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
            match ino {
                1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
                2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
                _ => reply.error(ENOENT),
            }
        }

        fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
            if ino == 2 {
                println!("WEREADDAFILE");
                reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
            } else {
                reply.error(ENOENT);
            }
        }

        fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
            if ino == 1 {
                if offset == 0 {
                    reply.add(1, 0, FileType::Directory, ".");
                    reply.add(1, 1, FileType::Directory, "..");
                    reply.add(2, 2, FileType::RegularFile, "hello.txt");
                }
                reply.ok();
            } else {
                reply.error(ENOENT);
            }
        }
    }

    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    fuse::mount(HelloFS, &mountpoint, &options).unwrap();
}

fn testNullFs() {
    env_logger::init();

    /*let path_string = match {
        Some(osstr_path) => {
            *//*let local_path_string = osstr_path.as_os_str().to_str().unwrap();
            if local_path_string == "" {
                panic!("Path was found but not able to parse it from OSStr. \
                    Maybe you are using some weird characters \
                    in the path that the program can"t parse?");
            }*//*

            //println!("{}", local_path_string);
            osstr_path;
        }
        None => {
            panic!("No path could be found in command arguments.");
        }
    };*/
    let path_string;
    path_string = match env::args_os().nth(1) {
        Some(working_value) => {
            working_value.to_os_string()
        }
        None => {
            panic!("NO PATH FOUND");
        }
    };
    let mountpoint = Path::new(&path_string);


    match fuse::mount(NullFS, &mountpoint, &[]) {
        Ok(T) => {
            println!("{:?}", T);
        }
        Err(err) => {
            panic!("called `Result::unwrap()` on an `Err` value: {:?}", err);
        }
    };
}