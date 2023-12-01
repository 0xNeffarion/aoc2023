use std::path::PathBuf;

pub fn read_input_lines(day: usize, n: usize) -> Vec<String> {
    let file_name = &format!("./input/day{}_{}.txt", day, n);
    let path = PathBuf::from(file_name);
    let content = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file {}", file_name));
    content.trim().lines().map(|s| s.to_string().trim().to_owned()).filter(|x| !x.is_empty()).collect()
}