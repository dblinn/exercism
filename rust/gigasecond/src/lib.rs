extern crate chrono;
extern crate time;
use chrono::*;

pub fn after(utc: DateTime<UTC>) -> DateTime<UTC> {
    utc + Duration::seconds(1000000000)
}
