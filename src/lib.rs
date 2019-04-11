#[macro_use]
extern crate lazy_static;

use {
    glob::glob, parking_lot::Mutex, rayon::prelude::*, regex::Regex, std::collections::HashMap,
    std::fs, walkdir::WalkDir,
};

lazy_static! {
    pub static ref POSSIBLE_ICONS: Mutex<HashMap<String, (Regex, u64)>> =
        Mutex::new(HashMap::new());
    static ref RE_ICONS: Regex =
        Regex::new(r#"([a-zA-Z0-9-]*):(\s*'\\e.*'|\s*'[a-zA-Z-]*.svg')"#).unwrap();
}

pub fn find_used_icons(sar: &str) {
    let search_path = format!("{}\\lib\\**\\*.*", sar);
    let _: Vec<_> = glob(search_path.as_ref())
        .unwrap()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|path| {
            let content = fs::read_to_string(path).unwrap();
            let mut possible_icons = POSSIBLE_ICONS.lock();
            for (_, val) in possible_icons.iter_mut() {
                for _ in val.0.find_iter(&content) {
                    val.1 += 1
                }
            }
        })
        .collect();
}

pub fn find_used_icons_seq(sar: &str) {
    let search_path = format!("{}\\lib", sar);
    for e in WalkDir::new(search_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let content = fs::read_to_string(e.path()).expect("Failed reading file!");
        let mut possible_icons = POSSIBLE_ICONS.lock();
        for (_, val) in possible_icons.iter_mut() {
            for _ in val.0.find_iter(&content) {
                val.1 += 1
            }
        }
    }
}

pub fn read_possible_icons(sar: &str) {
    let scss_file = format!("{}\\app\\styles\\core\\components\\_icons.scss", sar);
    let content = fs::read_to_string(scss_file).expect("Failed reading file!");

    let mut possible_icons = POSSIBLE_ICONS.lock();
    possible_icons.clear();
    for cap in RE_ICONS.captures_iter(content.as_ref()) {
        let icon = format!("icon-{}", &cap[1]);
        let regex = Regex::new(&icon).unwrap();
        possible_icons.insert(icon, (regex, 0));
    }
}
