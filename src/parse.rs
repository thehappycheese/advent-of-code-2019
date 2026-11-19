use std::{fs::read_to_string, path::Path};

pub fn read_input(path: &Path) -> String{
    read_to_string(path).expect("Could not open file").trim().to_string()
}
