pub mod exchange;
pub mod exchangeconfig;
pub mod genconfig;
pub mod genkeys;
pub mod man;
pub mod validate;

pub trait Command {
    fn run(self) -> anyhow::Result<()>;
}
