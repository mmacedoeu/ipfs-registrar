use crate::{cli::Cli, service};
use structopt::StructOpt;

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    service::upload(&cli)
}
