mod cli;
mod command;
mod service;

fn main() -> anyhow::Result<()> {
    command::run()
}
