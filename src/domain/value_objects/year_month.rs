//! YearMonth 値オブジェクト（予算管理の月単位）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    pub year: u16,
    pub month: u8,
}

impl YearMonth {
    pub fn new(year: u16, month: u8) -> Result<Self, YearMonthError> {
        if year < 2000 {
            return Err(YearMonthError::InvalidYear(year));
        }
        if !(1..=12).contains(&month) {
            return Err(YearMonthError::InvalidMonth(month));
        }
        Ok(Self { year, month })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum YearMonthError {
    #[error("year must be >= 2000, got {0}")]
    InvalidYear(u16),
    #[error("month must be 1-12, got {0}")]
    InvalidMonth(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_year_month() {
        assert!(YearMonth::new(2026, 6).is_ok());
    }

    #[test]
    fn invalid_month_13() {
        assert!(matches!(
            YearMonth::new(2026, 13),
            Err(YearMonthError::InvalidMonth(13))
        ));
    }
}
