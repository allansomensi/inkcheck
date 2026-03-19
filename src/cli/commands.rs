use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Scan {
        #[arg(short = 't', long, default_value_t = 5)]
        timeout: u64,
    },
}
