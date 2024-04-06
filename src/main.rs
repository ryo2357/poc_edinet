#[allow(unused_imports)]
use log::{debug, error};

mod verify;
use verify::verify_run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();
    dotenv::dotenv().ok();
    if let Err(r) = verify_run().await {
        error!("{{:?}}:{:?}", r);
        anyhow::bail!("error at running")
    };
    Ok(())
}
