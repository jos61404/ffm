use cli::Cli;
use console::style;
use progress::Progress;
use std::path::Path;

mod cli;
mod config;
mod file;
mod filter;
mod log;
mod progress;

fn main() -> std::io::Result<()> {
    // 進度條初始化
    let progress = Progress::new();
    let pb_path = progress.pb_path;
    let pb_config = progress.pb_config;
    let pb_file_list = progress.pb_file_list;
    let pb_copy_file = progress.pb_copy_file;
    let pb_filter_name = progress.pb_filter_name;
    let pb_filter_extension = progress.pb_filter_extension;

    // CLI 讀取資料
    let input = Cli::get_obj(pb_path)?;
    let original = input.original;
    let target = input.target;

    // 讀取設定
    let config = config::get(pb_config)?;

    // 讀取檔案路徑列表
    let ary = file::get_list(pb_file_list, Path::new(&original))?;

    // 過濾名稱
    let _filter_names = filter::name(pb_filter_name, &pb_copy_file, &target, &ary, &config.name)?;

    // 過濾副檔名
    let _filter_extensions = filter::extension(
        pb_filter_extension,
        &pb_copy_file,
        &target,
        &ary,
        &config.extension,
    )?;

    // 關閉資料搬移進度條
    pb_copy_file.set_message(&format!(
        "{} 資料搬移 : {}",
        style("[6/6]").bold().dim(),
        "完成"
    ));
    pb_copy_file.finish();

    Ok(())
}
