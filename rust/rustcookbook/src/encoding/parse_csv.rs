use std::{fmt::write, num::ParseIntError, ptr::read, str::FromStr};

use error_chain::error_chain;
use serde::Deserialize;

error_chain! {
    foreign_links {
        ParseInt(ParseIntError);
        Fmt(std::fmt::Error);
        Io(std::io::Error);
        UTF8(std::string::FromUtf8Error);
        CsvError(csv::Error);
        IntoInner(csv::IntoInnerError<csv::Writer<Vec<u8>>>);
    }
}

/// 基本使用和对象映射
pub fn csv_try_read() -> Result<()> {
    #[derive(Deserialize)]
    struct Record {
        year: u16,
        make: String,
        model: String,
        description: String,
    }
    // 注意换行
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        )
    }

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }
    Ok(())
}

pub fn csv_advanced() -> Result<()> {
    // use csv::invalid_option;

    #[derive(Debug, Deserialize)]
    struct Record {
        name: String,
        place: String,
        #[serde(deserialize_with = "csv::invalid_option")]
        id: Option<u64>,
    }

    let csv = "name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92
Ashley\tZurich\tsss";
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv.as_bytes());
    for record in reader.deserialize::<Record>() {
        println!("{:?}", record?);
    }

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        if record.iter().any(|x| x == "46") {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            wtr.write_record(&record)?;
        }
    }
    Ok(())
}

pub fn serde_records() -> Result<()> {
    use std::fs::File;
    use std::io::{BufWriter, Write};
    let f = File::create("csv.csv").map_or(File::open("csv.csv").unwrap(), |f| f);
    let f = BufWriter::new(f);
    let mut wtr = csv::Writer::from_writer(f);
    // let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;
    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}

pub fn hex_to_rgb() -> Result<()> {
    #[derive(Debug)]
    struct HexColor {
        red: u8,
        green: u8,
        blue: u8,
    }
    #[derive(Debug, Deserialize)]
    struct Row {
        color_name: String,
        color: HexColor,
    }

    impl FromStr for HexColor {
        type Err = Error;

        fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
            let trimmed = hex_color.trim_matches('#');
            if trimmed.len() != 6 {
                Err("Invalid length of hex string".into())
            } else {
                Ok(HexColor {
                    red: u8::from_str_radix(&trimmed[..2], 16)?,
                    green: u8::from_str_radix(&trimmed[2..4], 16)?,
                    blue: u8::from_str_radix(&trimmed[4..6], 16)?,
                })
            }
        }
    }

    use serde::{de, Deserializer};
    impl<'de> Deserialize<'de> for HexColor {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            FromStr::from_str(&s).map_err(de::Error::custom)
        }
    }

    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00sssssff"
        .to_owned();

    let mut wtr = csv::Writer::from_writer(vec![]);
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    for record in reader.deserialize::<Row>() {
        let record = record?;
        wtr.serialize((
            record.color_name,
            record.color.red,
            record.color.green,
            record.color.blue,
        ))?;
    }
    let vec = wtr.into_inner()?;
    let records = String::from_utf8_lossy(&vec);
    println!("-------------------------------------{}", records);

    Ok(())
}

#[test]
pub fn test() {
    csv_try_read();
    csv_advanced();
    serde_records();
    if let Err(ref errors) = hex_to_rgb() {
        eprintln!("Error level - description");
        errors
            .iter()
            .enumerate()
            .for_each(|(index, error)| println!("└> {} - {}", index, error));
        if let Some(backtrace) = errors.backtrace() {
            eprintln!("{:?}", backtrace);
        }
    }
}
