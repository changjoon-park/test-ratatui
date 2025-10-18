use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// FILETIME (Windows) 을 SystemTime으로 변환
fn filetime_to_systemtime(filetime: u64) -> SystemTime {
    // FILETIME epoch (1601-01-01) to Unix epoch (1970-01-01) 차이: 11644473600초
    const FILETIME_UNIX_DIFF: u64 = 11644473600;

    // 100나노초 단위를 초와 나노초로 분리
    let seconds = filetime / 10_000_000;
    let nanos = ((filetime % 10_000_000) * 100) as u32;

    // Unix epoch 기준으로 조정
    if seconds >= FILETIME_UNIX_DIFF {
        let unix_seconds = seconds - FILETIME_UNIX_DIFF;
        UNIX_EPOCH + Duration::new(unix_seconds, nanos)
    } else {
        UNIX_EPOCH
    }
}

/// 포렌식 타임스탬프 구조체
struct ForensicTimestamp {
    raw_filetime: u64,
    systime: SystemTime,
}

impl ForensicTimestamp {
    fn new(filetime: u64) -> Self {
        Self {
            raw_filetime: filetime,
            systime: filetime_to_systemtime(filetime),
        }
    }

    /// 여러 타임존으로 출력
    fn display_all_timezones(&self) {
        let utc: DateTime<Utc> = self.systime.into();

        println!("========================================");
        println!("포렌식 타임스탬프 분석");
        println!("========================================");
        println!("원본 FILETIME: {}", self.raw_filetime);
        println!("16진수:        0x{:016X}", self.raw_filetime);
        println!();

        // 주요 타임존들
        let timezones = vec![
            ("UTC (협정세계시)", chrono_tz::UTC),
            ("서울 (한국)", chrono_tz::Asia::Seoul),
            ("도쿄 (일본)", chrono_tz::Asia::Tokyo),
            ("뉴욕 (미국 동부)", chrono_tz::America::New_York),
            ("로스앤젤레스 (미국 서부)", chrono_tz::America::Los_Angeles),
            ("런던 (영국)", chrono_tz::Europe::London),
            ("파리 (프랑스)", chrono_tz::Europe::Paris),
        ];

        println!("주요 타임존들: {:?}", timezones);

        println!("타임존별 변환:");
        println!("{:-<70}", "");

        for (name, tz) in timezones {
            let local = utc.with_timezone(&tz);
            println!(
                "{:<25} | {}",
                name,
                local.format("%Y-%m-%d %H:%M:%S %Z (UTC%:z)")
            );
        }
        println!("{:-<70}", "");
    }

    /// 특정 타임존으로 출력
    fn display_in_timezone(&self, timezone: Tz) {
        let utc: DateTime<Utc> = self.systime.into();
        let local = utc.with_timezone(&timezone);

        println!("\n특정 타임존 출력 ({}):", timezone);
        println!("  {}", local.format("%Y년 %m월 %d일 %H:%M:%S %Z"));
    }
}

fn main() {
    println!("\n🔍 FILETIME → 타임존 변환 예제\n");

    // 예제 1: 실제 FILETIME 값
    println!("📌 예제 1: Windows FILETIME 분석");
    let filetime1 = 133738969230000000_u64; // 2024년 어느 시점
    let ts1 = ForensicTimestamp::new(filetime1);
    ts1.display_all_timezones();

    // 예제 2: 현재 시간
    println!("\n\n📌 예제 2: 현재 시간");
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    let current_filetime =
        (duration.as_secs() + 11644473600) * 10_000_000 + (duration.subsec_nanos() / 100) as u64;

    let ts2 = ForensicTimestamp::new(current_filetime);
    ts2.display_all_timezones();

    // 예제 3: 특정 타임존만 보기
    println!("\n\n📌 예제 3: 한국 시간으로만 보기");
    ts2.display_in_timezone(chrono_tz::Asia::Seoul);

    // 예제 4: 타임존 간 시차 확인
    println!("\n\n📌 예제 4: 타임존 간 시차");
    let utc: DateTime<Utc> = ts2.systime.into();
    let seoul = utc.with_timezone(&chrono_tz::Asia::Seoul);
    let ny = utc.with_timezone(&chrono_tz::America::New_York);

    println!("서울: {}", seoul.format("%H:%M:%S"));
    println!("뉴욕: {}", ny.format("%H:%M:%S"));

    // UTC offset 계산 (간단하게 포맷 문자열로)
    println!("서울 오프셋: {}", seoul.format("%:z"));
    println!("뉴욕 오프셋: {}", ny.format("%:z"));

    // 예제 5: 문자열에서 타임존 파싱
    println!("\n\n📌 예제 5: 동적 타임존 선택");
    let timezone_str = "Asia/Dubai";
    match timezone_str.parse::<Tz>() {
        Ok(tz) => {
            println!("타임존 '{}' 로드 성공!", timezone_str);
            ts2.display_in_timezone(tz);
        }
        Err(e) => println!("타임존 파싱 실패: {}", e),
    }

    println!("\n✅ 예제 완료!\n");
}
