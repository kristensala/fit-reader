use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fs::File;
use anyhow::Result;


#[derive(Debug)]
struct Session {
    total_time: f64,
    total_distance: f64,
    avg_power: f64
}

#[derive(Debug)]
struct  Item{
    name: String,
    value: Value
}

impl <'a>FromIterator<&'a FitDataField> for Session {
    fn from_iter<T: IntoIterator<Item = &'a FitDataField>>(iter: T) -> Self {

        let power_value: Vec<Item> = iter.into_iter()
            .filter(|x| x.name() == "avg_power" 
                || x.name() == "total_distance"
                || x.name() == "total_elapsed_time")
            .map(|x| {
                return Item {
                    name: x.name().to_string(),
                    value: x.value().to_owned()
                };
            })
            .collect();

        println!("{:?}", power_value);
        //let total_dist: f64 = Value::try_into(total_distance_value.to_owned()).unwrap();

        return Session {
            total_time: 12.2,
            total_distance: 12.1,
            avg_power: 12.1
        };
    }
}

fn main() {
    println!("Parsing FIT files using Profile version: {}", fitparser::profile::VERSION);
    let mut fp = File::open("/home/salakris/Downloads/salakris-2023-01-08-l2-up-down-150--157110383.fit").unwrap();
    for data in fitparser::from_reader(&mut fp) {
        // print the data in FIT file
        let session_data: Session = get_session_data(data).unwrap();
        println!("{:#?}", session_data);
    }
}

fn get_session_data(data: Vec<FitDataRecord>) -> Result<Session> {
    let session_data: Vec<FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::Session)
        .collect();

    let data_fields: Vec<&FitDataField> = session_data.first().unwrap().fields().into_iter().collect();
    let parsed_data = Session::from_iter(data_fields.into_iter());
    return Ok(parsed_data);
}
