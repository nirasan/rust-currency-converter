use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct EuroReferenceRate {
    Date: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    USD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    JPY: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    BGN: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    CYP: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    CZK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    DKK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    EEK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    GBP: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    HUF: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    LTL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    LVL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    MTL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    PLN: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ROL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    RON: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    SEK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    SIT: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    SKK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    CHF: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ISK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    NOK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    HRK: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    RUB: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    TRL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    TRY: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    AUD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    BRL: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    CAD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    CNY: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    HKD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    IDR: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ILS: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    INR: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    KRW: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    MXN: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    MYR: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    NZD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    PHP: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    SGD: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    THB: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    ZAR: Option<f64>,
}

#[test]
fn test() {
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
    }
    Ok(())
}
