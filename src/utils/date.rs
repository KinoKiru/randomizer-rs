use chrono::NaiveDate;

pub fn get_days_from_month(year: i32, m: u32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, m + 1, 1)
    }
    .expect("You baka imagine date error")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, m, 1).expect("lmao how this happen"))
    .num_days() as u32
}
