mod templates;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use templates::TemplateFinder;

pub fn build_templates(root_dir: &'static str) {
    // this is just a temp implementation that

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("templates.rs");
    let mut f = File::create(&dest_path).unwrap();

    let finder = TemplateFinder {
        root_dir: root_dir
    };

    let templates = finder.find_all();

    for template in templates {
        let mut tf = File::open(template.to_str().unwrap()).unwrap();

        let mut contents = String::new();
        tf.read_to_string(&mut contents).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }
}
