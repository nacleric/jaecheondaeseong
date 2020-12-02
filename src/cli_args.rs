use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliArgs {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}
