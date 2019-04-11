use find_used_icons_lib::{find_used_icons, read_possible_icons, POSSIBLE_ICONS};

const SAPIENT_APP_ROOT: &'static str = ".";

fn main() {
    read_possible_icons(SAPIENT_APP_ROOT);
    find_used_icons(SAPIENT_APP_ROOT);
    {
        let used_icons = POSSIBLE_ICONS.lock();
        for (key, val) in used_icons.iter() {
            if val.1 > 0 {
                println!("{} {}", key, val.1)
            }
        }
    }
}
