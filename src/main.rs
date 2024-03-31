#[allow(unused_imports)]
use log::{debug, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();
    dotenv::dotenv().ok();
    if let Err(r) = test_run().await {
        error!("{{:?}}:{:?}", r);
        anyhow::bail!("error at running")
    };
    Ok(())
}

#[allow(dead_code)]
async fn test_run_2() -> anyhow::Result<()> {
    use reqwest::Client;
    // https://api.edinet-fsa.go.jp/api/v2/documents.json?date=yyyy-mm-dd&type=2&Subscription-Key=xxxxx
    // 日付を指定して、提出文書のリストをダウンロード

    let key = std::env::var("EDINET_API")?;
    let client = Client::new();
    let url = "https://api.edinet-fsa.go.jp/api/v2/documents.json";
    let params = [("date", "2024-03-14"), ("Subscription-Key", &key)];
    let response = client.get(url).query(&params).send().await?; // 2
    let body = response.text().await?; // 3
    println!("{}", body);

    // "metadata": {
    //     "title": "提出された書類を把握するためのAPI",
    //     "parameter": {
    //       "date": "2024-03-14",
    //       "type": "1"
    //     },
    //     "resultset": {
    //       "count": 351
    //     },
    //     "processDateTime": "2024-03-31 00:01",
    //     "status": "200",
    //     "message": "OK"
    //   }
    // }

    Ok(())
}

#[allow(dead_code)]
async fn test_run_1() -> anyhow::Result<()> {
    // APIキーの取得
    let key = std::env::var("EDINET_API")?;
    println!("{:?}", key);
    Ok(())
}
