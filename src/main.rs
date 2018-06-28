extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha1;
extern crate sha2;

mod data;

use std::path::PathBuf;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.json".to_owned());
    let content = std::fs::read_to_string(path).expect("Cannot read config file");
    let mirror: data::Mirror = serde_json::from_str::<data::Config>(&content)
        .expect("Cannot parse config")
        .transform();
    std::fs::create_dir_all("./public/files").expect("Cannot create output direcotry");
    std::fs::write("./public/index.html", &format!("{}", mirror))
        .expect("Cannot write to output file");
    mirror
        .into_iter()
        .flat_map(IntoIterator::into_iter)
        .map(data::Issue::into_files)
        .flat_map(IntoIterator::into_iter)
        .for_each(|file| {
            let mut from = PathBuf::new();
            from.push("files");
            from.push(file.name());
            let mut to = PathBuf::new();
            to.push("public");
            to.push("files");
            to.push(file.name());
            std::fs::copy(from, to).unwrap_or_else(|_| panic!("Cannot copy {}", file.name()));
        });
}
