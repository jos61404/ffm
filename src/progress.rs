use console::style;
use indicatif::*;
use std::thread;
use ProgressBar;

#[derive(Debug)]
pub struct Progress {
    pub pb_path: ProgressBar,
    pub pb_config: ProgressBar,
    pub pb_file_list: ProgressBar,
    pub pb_filter_name: ProgressBar,
    pub pb_filter_extension: ProgressBar,
    pub pb_copy_file: ProgressBar,
}

impl Progress {
    pub fn new() -> Self {
        // 讀取路徑
        // -------------------------------------------------------------------
        let pb_path = ProgressBar::new(2);
        pb_path.set_style(
            ProgressStyle::default_bar()
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );

        // 讀取設定
        // -------------------------------------------------------------------
        let pb_config = ProgressBar::new(4);
        pb_config.set_style(
            ProgressStyle::default_bar()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );

        // 讀取檔案列表
        // -------------------------------------------------------------------
        let pb_file_list = ProgressBar::new(0);
        pb_file_list.set_style(
            ProgressStyle::default_bar()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );

        // 檔案名稱過濾
        // -------------------------------------------------------------------
        let pb_filter_name = ProgressBar::new(0);
        pb_filter_name.set_style(
            ProgressStyle::default_bar()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );
        // 檔案副檔名過濾
        // -------------------------------------------------------------------
        let pb_filter_extension = ProgressBar::new(0);
        pb_filter_extension.set_style(
            ProgressStyle::default_bar()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );

        // 檔案搬移
        // -------------------------------------------------------------------
        let pb_copy_file = ProgressBar::new(0);
        pb_copy_file.set_style(
            ProgressStyle::default_bar()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .progress_chars("##-")
                .template("[{elapsed_precise}] [{bar:50.cyan/blue}] {percent:<3}% ({eta}) {pos:>7}/{len:>7} {bytes:>10}/{total_bytes:>10} {wide_msg} \u{00A0}")
        );

        let multi_pb = MultiProgress::new();
        let pb_path = multi_pb.add(pb_path);
        let pb_config = multi_pb.add(pb_config);
        let pb_file_list = multi_pb.add(pb_file_list);
        let pb_filter_name = multi_pb.add(pb_filter_name);
        let pb_filter_extension = multi_pb.add(pb_filter_extension);
        let pb_copy_file = multi_pb.add(pb_copy_file);

        pb_path.set_message(&format!(
            "{} 路徑讀取 : {}",
            style("[1/6]").bold().dim(),
            "準備執行"
        ));

        pb_config.set_message(&format!(
            "{} 讀取設定 : {}",
            style("[2/6]").bold().dim(),
            "準備執行"
        ));

        pb_file_list.set_message(&format!(
            "{} 讀取檔案列表 : {}",
            style("[3/6]").bold().dim(),
            "準備執行"
        ));

        pb_filter_name.set_message(&format!(
            "{} 過濾名稱 : {}",
            style("[4/6]").bold().dim(),
            "準備執行"
        ));

        pb_filter_extension.set_message(&format!(
            "{} 過濾副名 : {}",
            style("[5/6]").bold().dim(),
            "準備執行"
        ));

        pb_copy_file.set_message(&format!(
            "{} 資料搬移 : {}",
            style("[6/6]").bold().dim(),
            "準備執行"
        ));

        thread::spawn(move || {
            multi_pb.join().expect("join");
        });

        Progress {
            pb_path,
            pb_config,
            pb_file_list,
            pb_filter_name,
            pb_filter_extension,
            pb_copy_file,
        }
    }
}
