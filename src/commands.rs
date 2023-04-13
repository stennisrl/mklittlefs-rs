pub mod pack;
pub mod unpack;
pub mod describe;

use clap::Subcommand;

pub trait CommandTrait{
    fn exec(&self) -> anyhow::Result<()>;
}