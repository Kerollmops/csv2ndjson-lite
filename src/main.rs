use std::collections::HashSet;
use std::io;
use std::io::Write;

use json::{object::Object, JsonValue};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "csv2ndjson-lite",
    about = "A little tool to convert a csv to a valid ndjson/json-stream."
)]
struct Opt {
    /// The list of fields to consider as comma separated values and output as arrays.
    #[structopt(long = "arrays", multiple = true)]
    array_fields: Vec<String>,

    /// The list of fields to consider as numbers.
    #[structopt(long = "numbers", multiple = true)]
    number_fields: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let array_fields: HashSet<_> = opt.array_fields.into_iter().collect();
    let number_fields: HashSet<_> = opt.number_fields.into_iter().collect();

    let mut wtr = io::stdout();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers()?.clone();

    for field in array_fields.r#union(&number_fields) {
        if !headers.iter().any(|f| f == field) {
            eprintln!("warning: field {:?} doesn't exist in the input CSV file", field);
        }
    }

    for result in rdr.into_records() {
        let record = result?;
        let mut object = Object::new();

        for (i, field) in record.iter().enumerate() {
            let header = headers.get(i).unwrap();
            let is_number = number_fields.contains(header);
            let is_array = array_fields.contains(header);
            let value = if is_array {
                let array = if is_number {
                    field
                        .split(',')
                        .filter_map(|text| text.trim().parse::<f64>().ok())
                        .map(JsonValue::from)
                        .collect()
                } else {
                    field.split(',').map(JsonValue::from).collect()
                };
                JsonValue::Array(array)
            } else if is_number {
                match field.parse::<f64>() {
                    Ok(number) => JsonValue::from(number),
                    Err(_) => continue,
                }
            } else {
                json::from(field)
            };
            object.insert(&header, value);
        }

        let _ = wtr
            .write_all(object.dump().as_bytes())
            .and_then(|_| writeln!(wtr));
    }

    Ok(())
}
