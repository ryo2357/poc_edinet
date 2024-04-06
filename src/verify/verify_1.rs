#[allow(unused_imports)]
use log::{debug, error, info};
use reqwest::Client;
use std::fs::File;
use std::io;

#[allow(dead_code)]
pub async fn verify_run_5() -> anyhow::Result<()> {
    let client = Client::new();
    // 書類取得 API
    // csv
    let key = std::env::var("EDINET_API")?;
    let url: String = format!(
        "https://api.edinet-fsa.go.jp/api/v2/documents/{}",
        // docID
        "S100T1U7"
    );
    println!("{}", url);
    let filename = String::new() + "output/" + url.split('/').last().unwrap() + "_csv.zip";
    let params = [
        // 1:xbrl, 2:pdf, 5:csv
        ("type", "5"),
        ("Subscription-Key", &key),
    ];
    let response = client.get(&url).query(&params).send().await?;

    let bytes = response.bytes().await?;
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

#[allow(dead_code)]
pub async fn verify_run_4() -> anyhow::Result<()> {
    let client = Client::new();
    // 書類取得 API
    // pdf
    let key = std::env::var("EDINET_API")?;
    let url: String = format!(
        "https://api.edinet-fsa.go.jp/api/v2/documents/{}",
        // docID
        "S100T1U7"
    );
    println!("{}", url);
    let filename = String::new() + "output/" + url.split('/').last().unwrap() + ".pdf";
    let params = [
        // 1:xbrl, 2:pdf, 5:csv
        ("type", "2"),
        ("Subscription-Key", &key),
    ];
    let response = client.get(&url).query(&params).send().await?;
    // let body = response.text().await?; // 3
    // info!("{}", body);

    let bytes = response.bytes().await?;
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

#[allow(dead_code)]
pub async fn verify_run_3() -> anyhow::Result<()> {
    let client = Client::new();
    // 書類取得 API
    // Zip
    let key = std::env::var("EDINET_API")?;
    let url: String = format!(
        "https://api.edinet-fsa.go.jp/api/v2/documents/{}",
        // docID
        "S100T1U7"
    );
    println!("{}", url);
    let filename = String::new() + "output/" + url.split('/').last().unwrap() + ".zip";
    let params = [
        // 1:xbrl, 2:pdf, 5:csv
        ("type", "5"),
        ("Subscription-Key", &key),
    ];
    let response = client.get(&url).query(&params).send().await?;
    // let body = response.text().await?; // 3
    // info!("{}", body);

    let bytes = response.bytes().await?;
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

#[allow(dead_code)]
pub async fn verify_run_2() -> anyhow::Result<()> {
    // https://api.edinet-fsa.go.jp/api/v2/documents.json?date=yyyy-mm-dd&type=2&Subscription-Key=xxxxx
    // 日付を指定して、提出文書のリストをダウンロード

    let key = std::env::var("EDINET_API")?;
    let client = Client::new();
    // 書類一覧 API
    let url = "https://api.edinet-fsa.go.jp/api/v2/documents.json";
    // 「ファイル日付」は当日以前　⇒　12の日付変更後にスクレイピングする？
    let params = [
        ("date", "2024-03-14"),
        ("type", "1"),
        ("Subscription-Key", &key),
    ];
    let response = client.get(url).query(&params).send().await?; // 2
    let body = response.text().await?; // 3
    info!("{}", body);

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

    // type:2だとresultsの中身が配列になる
    let params = [
        ("date", "2024-03-14"),
        ("type", "2"),
        ("Subscription-Key", &key),
    ];
    let response = client.get(url).query(&params).send().await?; // 2
    let body = response.text().await?; // 3
    info!("{}", body);

    // "results": [
    //     {
    //       "seqNumber": 1,
    //       "docID": "S100T1U7",
    //       "edinetCode": "E01509",
    //       "secCode": "61510",
    //       "JCN": "3010801009129",
    //       "filerName": "日東工器株式会社",
    //       "fundCode": null,
    //       "ordinanceCode": "010",
    //       "formCode": "170000",
    //       "docTypeCode": "220",
    //       "periodStart": null,
    //       "periodEnd": null,
    //       "submitDateTime": "2024-03-14 09:00",
    //       "docDescription": "自己株券買付状況報告書（法２４条の６第１項に基づくもの）",
    //       "issuerEdinetCode": null,
    //       "subjectEdinetCode": null,
    //       "subsidiaryEdinetCode": null,
    //       "currentReportReason": null,
    //       "parentDocID": null,
    //       "opeDateTime": null,
    //       "withdrawalStatus": "0",
    //       "docInfoEditStatus": "0",
    //       "disclosureStatus": "0",
    //       "xbrlFlag": "1",
    //       "pdfFlag": "1",
    //       "attachDocFlag": "0",
    //       "englishDocFlag": "0",
    //       "csvFlag": "1",1だとファイルが存在する
    //       "legalStatus": "1"
    //     },

    Ok(())
}

#[allow(dead_code)]
pub async fn verify_run_1() -> anyhow::Result<()> {
    // APIキーの取得
    let key = std::env::var("EDINET_API")?;
    println!("{:?}", key);
    Ok(())
}
