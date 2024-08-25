use rust_scheduling::model::agenda::Agenda;
use chrono::naive::NaiveDate;

fn main() {
    let year = 24;
    let month = 8;
    let day = 24;

    let date = NaiveDate::from_ymd_opt(year, month, day)
        .expect("Radiohaed man of war");
    
    
}
