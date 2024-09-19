use std::path::PathBuf;
use std::fs;
use std::process::Command;

pub fn get_package_info(path: &PathBuf) -> String {
    let metadata = fs::metadata(path).unwrap();
    let size = metadata.len() / 1024; // Size in KB
    let filename = path.file_name().unwrap().to_str().unwrap();

    format!("Filename: {}\nSize: {} KB", filename, size)
}

pub fn get_package_details(path: &PathBuf) -> String {
    let output = Command::new("dpkg")
        .arg("-I")
        .arg(path)
        .output()
        .expect("Failed to execute dpkg command");

    String::from_utf8_lossy(&output.stdout).to_string()
}
