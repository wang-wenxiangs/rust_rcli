mod base;
mod csvoptions;
use crate::base::{Options, SubCommand};
use crate::csvoptions::csv_process;
use clap::Parser;
fn main() -> anyhow::Result<()> {
    let option = Options::parse();
    match option.cmd {
        SubCommand::Csv(csv_option) => {
            csv_process(&csv_option.input, &csv_option.output)?;
        }
    }
    Ok(())
}

#[test]
fn my_test() {
    assert_eq!(1 + 1, 2);
}
