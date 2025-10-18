use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use chrono_tz::{America::New_York, Asia::Seoul, Europe::London};

fn main() {
    println!("=== chrono TimeZone Trait Use Cases ===\n");

    // Use Case 1: Creating datetime in specific timezone
    println!("1. Creating datetime in specific timezone:");
    let utc_dt = Utc.with_ymd_and_hms(2025, 9, 30, 12, 0, 0).unwrap();
    let seoul_dt = Seoul.with_ymd_and_hms(2025, 9, 30, 21, 0, 0).unwrap();
    println!("  UTC: {}", utc_dt);
    println!("  Seoul: {}\n", seoul_dt);

    // Use Case 2: Converting between timezones
    println!("2. Converting between timezones:");
    let utc_now = Utc::now();
    let in_seoul = utc_now.with_timezone(&Seoul);
    let in_ny = utc_now.with_timezone(&New_York);
    let in_london = utc_now.with_timezone(&London);
    println!("  UTC:    {}", utc_now.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("  Seoul:  {}", in_seoul.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("  NY:     {}", in_ny.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("  London: {}\n", in_london.format("%Y-%m-%d %H:%M:%S %Z"));

    // Use Case 3: Using fixed offset (when you only know UTC offset)
    println!("3. Using FixedOffset (UTC+9 for Korea):");
    let kst = FixedOffset::east_opt(9 * 3600).unwrap();
    let fixed_dt = kst.with_ymd_and_hms(2025, 9, 30, 21, 0, 0).unwrap();
    println!("  Fixed offset: {}\n", fixed_dt);

    // Use Case 4: Parsing strings with timezone
    println!("4. Parsing with timezone context:");
    let parsed_utc = Utc
        .datetime_from_str("2025-09-30 12:00:00", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    let parsed_seoul = Seoul
        .datetime_from_str("2025-09-30 21:00:00", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    println!("  Parsed UTC: {}", parsed_utc);
    println!("  Parsed Seoul: {}\n", parsed_seoul);

    // Use Case 5: Generic function accepting any TimeZone
    println!("5. Generic function with TimeZone trait:");
    print_time_in_timezone(&Utc);
    print_time_in_timezone(&Seoul);
    print_time_in_timezone(&Local);
    println!();

    // Use Case 6: Timestamp to timezone-aware datetime
    println!("6. Converting timestamp to specific timezone:");
    let timestamp = 1727697600; // Unix timestamp
    let utc = Utc.timestamp_opt(timestamp, 0).unwrap();
    let seoul = Seoul.timestamp_opt(timestamp, 0).unwrap();
    println!("  Timestamp {} in UTC: {}", timestamp, utc);
    println!("  Timestamp {} in Seoul: {}\n", timestamp, seoul);

    // Use Case 7: Working with Local timezone
    println!("7. System local timezone:");
    let local_now = Local::now();
    let as_utc = local_now.with_timezone(&Utc);
    println!("  Local: {}", local_now.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("  As UTC: {}", as_utc.format("%Y-%m-%d %H:%M:%S %Z"));
}

// Generic function that works with any TimeZone implementation
fn print_time_in_timezone<Tz: TimeZone>(tz: &Tz)
where
    Tz::Offset: std::fmt::Display,
{
    let now = Utc::now().with_timezone(tz);
    println!("  Current time: {}", now.format("%H:%M:%S %Z"));
}

// Practical example: Meeting scheduler
fn schedule_meeting(utc_time: DateTime<Utc>) {
    println!("\n=== Meeting scheduled for ===");
    println!("Seoul:  {}", utc_time.with_timezone(&Seoul).format("%Y-%m-%d %H:%M"));
    println!("NY:     {}", utc_time.with_timezone(&New_York).format("%Y-%m-%d %H:%M"));
    println!("London: {}", utc_time.with_timezone(&London).format("%Y-%m-%d %H:%M"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_conversion_preserves_instant() {
        let utc = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let seoul = utc.with_timezone(&Seoul);

        // Different representation, same instant
        assert_eq!(seoul.hour(), 9);
        assert_eq!(utc.timestamp(), seoul.timestamp());
    }

    #[test]
    fn test_fixed_offset_equivalence() {
        let named = Seoul.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap();
        let fixed = FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .with_ymd_and_hms(2025, 1, 1, 9, 0, 0)
            .unwrap();

        assert_eq!(named.timestamp(), fixed.timestamp());
    }
}