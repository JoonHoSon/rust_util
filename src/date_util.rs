//! 날짜 관련 함수 모음

use crate::error::InvalidArgumentError;
use chrono::{DateTime, NaiveDateTime, Offset, TimeZone, Utc};
use chrono_tz::Tz;

/// 지정된 날짜 및 시간 문자열을 UTC 날짜로 변경
///
/// 문자열 형태로 전달되는 날짜 및 시간 정보를 **UTC** 시간대로 변환하여 반환.
///
/// # Arguments
///
/// - `datetime` - 날짜 및 시간 문자열 (e.g. '2024-11-27 13:23:47')
/// - `pattern` - 날짜 및 시간 패턴 (e.g. '%Y-%m-%d %H:%M:%S')
/// - `timezone` - [Tz]에서 정의된 timezone 정보 (e.g. [Tz::Asia__Seoul])
///
/// # Return
///
/// - 변환 결과 `Result<DateTime<Utc>, InvalidArgumentError>`
///
/// # Link
///
/// - [NaiveDateTime::parse_from_str]
/// - [Tz::offset_from_utc_datetime]
/// - [chrono_tz::TzOffset::fix]
/// - [Utc::from_utc_datetime]
///
/// # Errors
///
/// - [InvalidArgumentError] - 잘못된 날짜 및 시간 형식 혹은 패턴
///
/// # Example
///
/// ```rust
/// use chrono_tz::Tz;
/// use chrono::{DateTime, Datelike, Timelike};
/// use cliff3_util::date_util::local_datetime_to_utc;
///
/// // KST 2024-11-22 10:29:48
/// // UTC 2024-11-22 01:29:48
/// let datetime = "20241122102948";
/// let pattern = "%Y%m%d%H%M%S";
/// let timezone = Tz::Asia__Seoul;
/// let result = local_datetime_to_utc(datetime, pattern, &timezone);
///
/// assert!(result.is_ok());
///
/// let result = result.unwrap();
///
/// assert_eq!(2024, result.year());
/// assert_eq!(11, result.month());
/// assert_eq!(22, result.day());
/// assert_eq!(1, result.hour());
/// assert_eq!(29, result.minute());
/// assert_eq!(48, result.second());
/// ```
pub fn local_datetime_to_utc(
    datetime: &str,
    pattern: &str,
    timezone: &Tz,
) -> Result<DateTime<Utc>, InvalidArgumentError> {
    let naive_datetime = NaiveDateTime::parse_from_str(datetime, pattern);

    if naive_datetime.is_err() {
        let err = naive_datetime.as_ref().unwrap_err();

        println!("parse_from_str error => {:#?}", err);

        return Err(InvalidArgumentError::new(format!("{err:#?}").as_ref()));
    }

    Ok({
        let offset = timezone.offset_from_utc_datetime(naive_datetime.as_ref().unwrap());
        let fixed = offset.fix();

        Utc.from_utc_datetime(
            &fixed
                .from_local_datetime(naive_datetime.as_ref().unwrap())
                .unwrap()
                .naive_utc(),
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::date_util::local_datetime_to_utc;
    use chrono::{Datelike, Timelike};
    use chrono_tz::Tz;

    #[test]
    fn local_datetime_to_utc_test() {
        // KST 2024-11-22 09:54:45
        // UTC 2024-11-22 00:54:45
        let str_datetime = "20241122095445"; // 2024-11-22 09:54:45
        let pattern = "%Y%m%d%H%M%S";
        let timezone = Tz::Asia__Seoul;

        let result = local_datetime_to_utc(str_datetime, pattern, &timezone);

        assert!(
            result.is_ok(),
            "{}",
            format!("변환 실패 : {:#?}", result.as_ref().unwrap_err())
        );

        let result = result.unwrap();

        println!("utc result => {:#?}", result);

        assert_eq!(2024, result.year());
        assert_eq!(11, result.month());
        assert_eq!(22, result.day());
        assert_eq!(0, result.hour());
        assert_eq!(54, result.minute());
        assert_eq!(45, result.second());
    }
}
