use std::{fs::File, io::Read, path::Path};

pub fn get_input(year: usize, day: usize, is_test: bool, buffer: &mut String) {
    let file_name = if is_test { "test.txt" } else { "input.txt" };
    let relative_path = format!("src/year{}/day{}/{}", year, day, file_name);
    let path = Path::new(&relative_path);
    
    let mut file = File::open(path).unwrap();
    file.read_to_string(buffer).unwrap();
}
