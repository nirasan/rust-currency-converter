use std::error::Error;
use std::fs::File;
use chrono::prelude::*;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
struct EuroReferenceRate {
    #[serde(rename = "Date")]
    date: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "USD")]
    usd: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "JPY")]
    jpy: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "BGN")]
    bgn: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "CYP")]
    cyp: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "CZK")]
    czk: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "DKK")]
    dkk: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "EEK")]
    eek: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "GBP")]
    gbp: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "HUF")]
    huf: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "LTL")]
    ltl: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "LVL")]
    lvl: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "MTL")]
    mtl: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "PLN")]
    pln: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "ROL")]
    rol: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "RON")]
    ron: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "SEK")]
    sek: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "SIT")]
    sit: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "SKK")]
    skk: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "CHF")]
    chf: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "ISK")]
    isk: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "NOK")]
    nok: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "HRK")]
    hrk: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "RUB")]
    rub: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "TRL")]
    trl: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "TRY")]
    try_: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "AUD")]
    aud: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "BRL")]
    brl: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "CAD")]
    cad: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "CNY")]
    cny: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "HKD")]
    hkd: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "IDR")]
    idr: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "ILS")]
    ils: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "INR")]
    inr: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "KRW")]
    krw: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "MXN")]
    mxn: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "MYR")]
    myr: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "NZD")]
    nzd: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "PHP")]
    php: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "SGD")]
    sgd: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "THB")]
    thb: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    #[serde(rename = "ZAR")]
    zar: Option<f64>,
}

#[test]
fn test_run() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<(), Box<Error>> {
    let data = "\
Date,USD,JPY,BGN,CYP,CZK,DKK,EEK,GBP,HUF,LTL,LVL,MTL,PLN,ROL,RON,SEK,SIT,SKK,CHF,ISK,NOK,HRK,RUB,TRL,TRY,AUD,BRL,CAD,CNY,HKD,IDR,ILS,INR,KRW,MXN,MYR,NZD,PHP,SGD,THB,ZAR,
2019-04-10,1.1279,125.38,1.9558,N/A,25.608,7.4652,N/A,0.86083,321.84,N/A,N/A,N/A,4.2833,N/A,4.7605,10.44,N/A,N/A,1.128,134.2,9.5895,7.4253,72.86,N/A,6.4144,1.5785,4.3301,1.5032,7.5758,8.8405,15959.79,4.0351,77.9945,1283.09,21.2783,4.6328,1.6697,58.543,1.5258,35.839,15.7391,
2019-04-09,1.1277,125.51,1.9558,N/A,25.618,7.465,N/A,0.86335,321.5,N/A,N/A,N/A,4.287,N/A,4.761,10.425,N/A,N/A,1.127,133.8,9.619,7.4342,73.0029,N/A,6.4056,1.5781,4.3471,1.4984,7.5688,8.844,15937.78,4.0346,78.188,1284.96,21.3479,4.622,1.6718,58.682,1.5255,35.804,15.8215,
2019-04-08,1.1246,125.36,1.9558,N/A,25.634,7.4651,N/A,0.86183,321.54,N/A,N/A,N/A,4.2887,N/A,4.7512,10.4325,N/A,N/A,1.1245,133.6,9.6305,7.4318,73.268,N/A,6.3781,1.5823,4.3558,1.5042,7.5561,8.8247,15927.71,4.0255,78.3215,1288.11,21.439,4.6123,1.6694,58.633,1.5243,35.942,15.8562,
";
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    for result in reader.deserialize() {
        let record: EuroReferenceRate = result?;
        println!("{:?}", record);

        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
        println!("{:?}", date);
    }
    Ok(())
}

#[test]
fn test_embed() -> Result<(), Box<Error>> {
    let bytes = include_bytes!("eurofxref-hist.csv");
    let mut reader = csv::Reader::from_reader(bytes as &[u8]);
    for result in reader.deserialize() {
        let record: EuroReferenceRate = result?;
        println!("{:?}", record);

        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
        println!("{:?}", date);
    }
    Ok(())
}

#[test]
fn test_zip() {
    use std::io::prelude::*;

    let bytes = include_bytes!("eurofxref-hist.zip");
    let bytes: &[u8] = bytes;
    let mut cur = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cur).unwrap();

    let mut file = match archive.by_name("eurofxref-hist.csv")
        {
            Ok(file) => file,
            Err(..) => { println!("File test/lorem_ipsum.txt not found"); return; }
        };

//    let mut contents = String::new();
//    file.read_to_string(&mut contents).unwrap();
//    println!("{}", contents);

    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: EuroReferenceRate = result.unwrap();
        println!("{:?}", record);

        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
        println!("{:?}", date);
    }
}

#[test]
fn test_reqwest() {
    let mut res = reqwest::get("https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip").unwrap();
    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf).unwrap();
    println!("{:?}", buf);

    let bytes: &[u8] = &buf;
    let mut cur = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cur).unwrap();

    let mut file = match archive.by_name("eurofxref-hist.csv")
        {
            Ok(file) => file,
            Err(..) => { println!("File test/lorem_ipsum.txt not found"); return; }
        };
    
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: EuroReferenceRate = result.unwrap();
        println!("{:?}", record);

        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
        println!("{:?}", date)
    }
}
