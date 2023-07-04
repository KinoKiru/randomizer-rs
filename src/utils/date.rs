use chrono::NaiveDate;

// TODO fix this lmao
pub fn get_days_from_month(year: i32, m: u32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, m + 1, 1)
    }
    .signed_duration_since(NaiveDate::from_ymd(year, m, 1))
    .num_days() as u32
}
