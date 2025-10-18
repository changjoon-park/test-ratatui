use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

fn main() {
    println!("=== SystemTime Examples ===\n");

    // Current time
    let now = SystemTime::now();
    println!("Current time: {:?}", now);

    // Duration since UNIX epoch (Jan 1, 1970)
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    println!("Seconds since UNIX epoch: {}", since_epoch.as_secs());
    println!("Milliseconds since UNIX epoch: {}\n", since_epoch.as_millis());

    // Time arithmetic
    let one_hour_later = now + Duration::from_secs(3600);
    let one_day_ago = now - Duration::from_secs(86400);

    println!("One hour from now: {:?}", one_hour_later);
    println!("One day ago: {:?}\n", one_day_ago);

    // Measuring elapsed time
    let start = SystemTime::now();
    println!("Starting operation...");
    thread::sleep(Duration::from_millis(100));

    let elapsed = start.elapsed().unwrap();
    println!("Operation took: {:?}\n", elapsed);

    // Comparing times
    let time1 = SystemTime::now();
    thread::sleep(Duration::from_millis(50));
    let time2 = SystemTime::now();

    match time2.duration_since(time1) {
        Ok(diff) => println!("Time difference: {:?}", diff),
        Err(e) => println!("Error: {:?}", e),
    }

    // Creating specific time offsets
    let future = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 7); // 7 days
    println!("\n7 days from now: {:?}", future);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_comparison() {
        let earlier = SystemTime::now();
        thread::sleep(Duration::from_millis(10));
        let later = SystemTime::now();

        assert!(later > earlier);
        assert!(later.duration_since(earlier).is_ok());
    }

    #[test]
    fn test_duration_arithmetic() {
        let now = SystemTime::now();
        let future = now + Duration::from_secs(100);

        let diff = future.duration_since(now).unwrap();
        assert_eq!(diff.as_secs(), 100);
    }

    #[test]
    fn test_unix_epoch() {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

        // Should be sometime after 2020
        assert!(since_epoch.as_secs() > 1_577_836_800);
    }
}