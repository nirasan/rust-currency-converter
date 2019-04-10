use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct EuroReferenceRate {
    Date: String, //chrono::Date<chrono::Utc>,
    USD: f64,
    JPY: f64,
    BGN: f64,
    CYP: f64,
    CZK: f64,
    DKK: f64,
    EEK: f64,
    GBP: f64,
    HUF: f64,
    LTL: f64,
    LVL: f64,
    MTL: f64,
    PLN: f64,
    ROL: f64,
    RON: f64,
    SEK: f64,
    SIT: f64,
    SKK: f64,
    CHF: f64,
    ISK: f64,
    NOK: f64,
    HRK: f64,
    RUB: f64,
    TRL: f64,
    TRY: f64,
    AUD: f64,
    BRL: f64,
    CAD: f64,
    CNY: f64,
    HKD: f64,
    IDR: f64,
    ILS: f64,
    INR: f64,
    KRW: f64,
    MXN: f64,
    MYR: f64,
    NZD: f64,
    PHP: f64,
    SGD: f64,
    THB: f64,
    ZAR: f64,
}

#[test]
fn test() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<(), Box<Error>> {
    let file = File::open("./eurofxref-hist.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: EuroReferenceRate = result?;
        println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }
    Ok(())
}
