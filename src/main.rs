#![allow(unused_imports)]
#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Read;
use std::fs::File;
use md5::{Digest, Md5};
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::cell::RefCell;
use chrono::{DateTime, NaiveDateTime, Utc};

const DATA_DIR: &str = "data";

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Manifest {
    filename: String,
    hash: String,
    created_at: String,
}

fn md5_hash(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut hasher = Md5::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    let hash_string = format!("{:x}", hash_result);

    hash_string
}

fn timestamp(path: &Path) -> String {
    let created = fs::metadata(path)
        .unwrap()
        .created()
        .unwrap()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap();

    let naive_datetime = NaiveDateTime::from_timestamp_opt(created.as_secs() as i64, created.subsec_nanos())
        .expect("Invalid timestamp");
    let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    datetime_utc.format("%Y-%m-%dT%H:%M:%S%.6f").to_string()
}

fn file_info(path: &Path) -> Manifest {
    Manifest {
        filename: path.to_str().unwrap().to_string(),
        hash: md5_hash(path),
        created_at: timestamp(path),
    }
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[get("/manifest")]
fn get_manifest() -> Value {
    let manifest: RefCell<Vec<Manifest>> = RefCell::new(Vec::new());

    visit_dirs(Path::new(DATA_DIR), &|entry: &DirEntry| {
        manifest.borrow_mut().push(file_info(&entry.path()));
    })
    .unwrap();

    json!(manifest.into_inner())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_manifest])
}
