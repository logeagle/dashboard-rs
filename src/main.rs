#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Row;

#[derive(Serialize)]
struct LogData {
    timestamp: String,
    // Add other fields you want to display
}

#[get("/")]
fn index() -> Template {
    let access_logs = read_log_data("access.parquet");
    let error_logs = read_log_data("error.parquet");
    let context = LogContext { access_logs, error_logs };
    Template::render("index", &context)
}

fn read_log_data(file_path: &str) -> Vec<LogData> {
    let file = std::fs::File::open(file_path).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();

    let mut logs = Vec::new();
    let mut iter = reader.get_row_iter(None).unwrap();

    while let Some(record) = iter.next() {
        let row = record.unwrap();
        let log_data = LogData {
            timestamp: row.get_string(0).unwrap().to_string(),
            // Parse other fields as needed
        };
        logs.push(log_data);
    }

    logs
}

#[derive(Serialize)]
struct LogContext {
    access_logs: Vec<LogData>,
    error_logs: Vec<LogData>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
}
