use console::style;
use indicatif::ProgressBar;
use structopt::StructOpt;

#[warn(non_snake_case)]
#[derive(Debug, StructOpt)]
#[structopt(name = "ffm")]
pub struct Cliget {
    #[structopt(short = "o", long = "original")]
    pub original: Option<String>,

    #[structopt(short = "t", long = "target")]
    pub target: Option<String>,
}

#[derive(Debug)]
pub struct Cli {
    pub original: String,
    pub target: String,
}

impl Cli {
    pub fn get_obj(pb_path: ProgressBar) -> std::io::Result<Self> {
        pb_path.set_message(&format!(
            "{} 讀取路徑 : {}",
            style("[1/6]").bold().dim(),
            "執行中..."
        ));
        let input = Cliget::from_args();

        // 來源路徑
        let original = match input.original {
            Some(input_original) => input_original.to_string(),
            None => panic!("尚未輸入來源路徑"),
        };
        pb_path.inc(1);

        // 目標路徑
        let target = match input.target {
            Some(input_target) => input_target.to_string(),
            None => panic!("尚未輸入目標路徑"),
        };
        pb_path.inc(1);

        pb_path.set_message(&format!(
            "{} 讀取路徑 : {}",
            style("[1/6]").bold().dim(),
            "完成"
        ));
        pb_path.finish();

        return Ok(Cli { original, target });
    }
}
