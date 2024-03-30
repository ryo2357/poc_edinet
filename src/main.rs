#[allow(unused_imports)]
use log::{debug, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();
    dotenv::dotenv().ok();
    // if let Err(r) = running().await {
    //     error!("{{:?}}:{:?}", r);
    //     anyhow::bail!("error at running")
    // };
    Ok(())
}
