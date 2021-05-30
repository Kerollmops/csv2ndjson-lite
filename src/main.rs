use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::{env, io};

use json::{object::Object, JsonValue};

fn try_main() -> Result<(), Box<dyn Error>> {
    let array_fields: HashSet<_> = env::args().collect();

    let mut wtr = io::stdout();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers()?.clone();

    for result in rdr.into_records() {
        let record = result?;
        let mut object = Object::new();

        for (i, field) in record.iter().enumerate() {
            let header = headers.get(i).unwrap();
            let value = if array_fields.contains(header) {
                JsonValue::Array(field.split(',').map(JsonValue::from).collect())
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

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}
