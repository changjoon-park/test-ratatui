use chrono_tz::Tz;
use std::mem;

fn main() {
    println!("=== Tz 타입 분석 ===\n");

    // Tz는 어떤 타입인가?
    let seoul = chrono_tz::Asia::Seoul;
    let tokyo = chrono_tz::Asia::Tokyo;
    let utc = chrono_tz::UTC;

    println!("1. Tz 크기:");
    println!("   sizeof(Tz) = {} bytes", mem::size_of::<Tz>());
    println!("   sizeof(String) = {} bytes (비교용)", mem::size_of::<String>());
    println!("   sizeof(&str) = {} bytes (비교용)", mem::size_of::<&str>());
    println!();

    // Tz는 Display 구현
    println!("2. Display 출력:");
    println!("   seoul = {}", seoul);
    println!("   tokyo = {}", tokyo);
    println!("   utc = {}", utc);
    println!();

    // Tz는 Debug 구현
    println!("3. Debug 출력:");
    println!("   seoul = {:?}", seoul);
    println!("   tokyo = {:?}", tokyo);
    println!("   utc = {:?}", utc);
    println!();

    // Tz는 Copy, Clone
    println!("4. Copy/Clone 가능:");
    let seoul_copy = seoul; // Copy
    let seoul_clone = seoul.clone(); // Clone
    println!("   seoul_copy = {}", seoul_copy);
    println!("   seoul_clone = {}", seoul_clone);
    println!("   원본 seoul = {} (여전히 사용 가능!)", seoul);
    println!();

    // Tz는 PartialEq
    println!("5. 비교 가능:");
    println!("   seoul == tokyo? {}", seoul == tokyo);
    println!("   seoul == seoul_copy? {}", seoul == seoul_copy);
    println!();

    // 문자열에서 파싱
    println!("6. 문자열 파싱:");
    let parsed: Result<Tz, _> = "Asia/Seoul".parse();
    match parsed {
        Ok(tz) => {
            println!("   \"Asia/Seoul\".parse() = {}", tz);
            println!("   parsed == seoul? {}", tz == seoul);
        }
        Err(e) => println!("   파싱 실패: {}", e),
    }
    println!();

    // Tz는 enum!
    println!("7. Tz는 enum입니다:");
    println!("   모든 IANA 타임존이 enum variant로 정의됨");
    println!("   예: Tz::Asia__Seoul, Tz::UTC 등");
    println!();

    // 내부 데이터
    println!("8. 내부 데이터 (타임존 정보):");
    use chrono::{Utc, TimeZone};
    let now = Utc::now();
    let seoul_time = seoul.from_utc_datetime(&now.naive_utc());
    let tokyo_time = tokyo.from_utc_datetime(&now.naive_utc());

    println!("   같은 UTC 시간을 변환:");
    println!("   UTC:   {}", now.format("%H:%M:%S"));
    println!("   Seoul: {} (offset: {})",
        seoul_time.format("%H:%M:%S"),
        seoul_time.format("%:z")
    );
    println!("   Tokyo: {} (offset: {})",
        tokyo_time.format("%H:%M:%S"),
        tokyo_time.format("%:z")
    );
    println!();

    println!("✅ Tz는 문자열이 아니라 타임존 규칙을 담은 enum입니다!");
}
