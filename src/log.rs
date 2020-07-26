use std::fs;
use std::{io::Write, path::Path};

pub fn add(original_path: &Path, file_path: String, file_name: String) -> std::io::Result<()> {
    let mut log_path = String::from(&file_path);
    log_path.push_str("/log.txt");

    let mut log_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(log_path)?;

    let mut text = String::from("----------------------------------------------------------\n");
    text.push_str("來源位置 ： ");
    text.push_str(original_path.to_str().expect("轉換錯誤"));
    text.push_str("\n隔離路徑 ： ");
    text.push_str(&file_path);
    text.push_str("\n檔案名稱 ： ");
    text.push_str(&file_name);
    text.push_str("\n");
    log_file.write_all(text.as_bytes()).expect("轉換錯誤");

    return Ok(());
}
