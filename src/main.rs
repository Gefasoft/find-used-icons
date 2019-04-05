#[macro_use]
extern crate lazy_static;

use {parking_lot::Mutex, regex::Regex, std::collections::HashMap, std::fs, walkdir::WalkDir};

lazy_static! {
    static ref POSSIBLE_ICONS: Mutex<HashMap<String, (Regex, u64)>> = Mutex::new(HashMap::new());
    static ref RE_ICONS: Regex =
        Regex::new(r#"([a-zA-Z0-9-]*):(\s*'\\e.*'|\s*'[a-zA-Z-]*.svg')"#).unwrap();
}

fn main() {
    read_possible_icons();
    find_used_icons();
    {
        let used_icons = POSSIBLE_ICONS.lock();
        for (key, val) in used_icons.iter() {
            if val.1 > 0 {
                println!("{} {}", key, val.1)
            }
        }
    }
}

fn find_used_icons() {
    let mut possible_icons = POSSIBLE_ICONS.lock();
    for e in WalkDir::new(r#".\lib"#)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let content = fs::read_to_string(e.path()).expect("Failed reading file!");
        for (_, val) in possible_icons.iter_mut() {
            for _ in val.0.find_iter(&content) {
                val.1 += 1
            }
        }
    }
}

fn read_possible_icons() {
    let path = r#".\app\styles\core\components\_icons.scss"#;
    let content = fs::read_to_string(path).expect("Failed reading file!");

    let mut possible_icons = POSSIBLE_ICONS.lock();
    for cap in RE_ICONS.captures_iter(content.as_ref()) {
        let icon = format!("icon-{}", &cap[1]);
        let regex = Regex::new(&icon).unwrap();
        possible_icons.insert(icon, (regex, 0));
    }
}
