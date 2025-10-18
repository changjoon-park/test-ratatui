use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use chrono_tz::Asia::Seoul;

fn main() {
    println!("=== Chrono Test Examples ===\n");

    // Current time
    let now_utc: DateTime<Utc> = Utc::now();
    let now_local: DateTime<Local> = Local::now();

    println!("Current UTC time: {}", now_utc);
    println!("Current local time: {}", now_local);
    println!("Formatted: {}\n", now_local.format("%Y-%m-%d %H:%M:%S"));

    // Parse a date
    let date = NaiveDate::from_ymd_opt(2025, 9, 30).unwrap();
    println!("Parsed date: {}", date);
    println!("Day of week: {}\n", date.format("%A"));

    // Timezone conversion
    let utc_time = Utc::now();
    let seoul_time = utc_time.with_timezone(&Seoul);

    println!("UTC: {}", utc_time.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("Seoul: {}", seoul_time.format("%Y-%m-%d %H:%M:%S %Z"));

    // Duration arithmetic
    let future = now_local + chrono::Duration::days(7);
    println!("\n7 days from now: {}", future.format("%Y-%m-%d"));

    let past = now_local - chrono::Duration::hours(24);
    println!("24 hours ago: {}", past.format("%Y-%m-%d %H:%M:%S"));
}
