use chrono::{Datelike, NaiveDate};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

// ── Is this a leap year? ──────────────────────────────────────
// Rule: div by 4, EXCEPT centuries, UNLESS div by 400
// Most apps skip this. We don't.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn days_in_year(year: i32) -> Decimal {
    if is_leap_year(year) { dec!(366) } else { dec!(365) }
}

// ── Results we return ─────────────────────────────────────────
pub struct YearBreakdown {
    pub year: i32,
    pub days_accrued: i64,
    pub days_in_year: i64,
    pub interest_this_year: Decimal,
    pub balance_end_of_year: Decimal,
}

pub struct InterestResult {
    pub principal: Decimal,
    pub annual_rate: Decimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: i64,
    pub interest_earned: Decimal,
    pub final_balance: Decimal,
    pub year_breakdown: Vec<YearBreakdown>,
}

// ── The engine: calendar-aware compound interest ──────────────
// Every year is processed individually.
// Leap years divide by 366. Normal years by 365.
// This is the difference between a student and a professional.
pub fn compound_interest_calendar_aware(
    principal: Decimal,
    annual_rate_percent: Decimal,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> InterestResult {
    let annual_rate = annual_rate_percent / dec!(100);
    let total_days = (end_date - start_date).num_days();
    let mut balance = principal;
    let mut year_breakdown = Vec::new();
    let mut current_date = start_date;

    while current_date < end_date {
        let current_year = current_date.year();
        let year_end = NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap();
        let period_end = if year_end < end_date { year_end } else { end_date };
        let days_this_period = (period_end - current_date).num_days();

        if days_this_period == 0 { break; }

        let days_in_yr = days_in_year(current_year);
        let days_dec = Decimal::from(days_this_period);
        let daily_rate = annual_rate / days_in_yr;
        let growth = (dec!(1) + daily_rate).powd(days_dec);
        let interest = balance * growth - balance;
        let new_balance = balance + interest;

        year_breakdown.push(YearBreakdown {
            year: current_year,
            days_accrued: days_this_period,
            days_in_year: if is_leap_year(current_year) { 366 } else { 365 },
            interest_this_year: interest.round_dp(6),
            balance_end_of_year: new_balance.round_dp(6),
        });

        balance = new_balance;
        current_date = NaiveDate::from_ymd_opt(current_year + 1, 1, 1).unwrap();
    }

    InterestResult {
        principal,
        annual_rate: annual_rate_percent,
        start_date,
        end_date,
        total_days,
        interest_earned: (balance - principal).round_dp(4),
        final_balance: balance.round_dp(4),
        year_breakdown,
    }
}

// ── Naive calculation — what broken apps do ───────────────────
// Always divides by 365. We use this only to SHOW the error.
pub fn compound_interest_naive(
    principal: Decimal,
    annual_rate_percent: Decimal,
    total_days: i64,
) -> Decimal {
    let rate = annual_rate_percent / dec!(100);
    let daily_rate = rate / dec!(365);
    let days = Decimal::from(total_days);
    (principal * (dec!(1) + daily_rate).powd(days)).round_dp(4)
}

// ── Float proof ───────────────────────────────────────────────
pub struct FloatProof {
    pub decimal_result: Decimal,
    pub float_result: f64,
    pub discrepancy: Decimal,
    pub operations: u32,
}

pub fn prove_float_error(operations: u32) -> FloatProof {
    let mut decimal_acc = dec!(0);
    let mut float_acc: f64 = 0.0;

    for _ in 0..operations {
        decimal_acc += dec!(0.1);
        float_acc += 0.1_f64;
    }

    let float_as_decimal = Decimal::try_from(float_acc).unwrap_or(dec!(0));
    let discrepancy = (decimal_acc - float_as_decimal).abs();

    FloatProof {
        decimal_result: decimal_acc,
        float_result: float_acc,
        discrepancy,
        operations,
    }
}

// ── Tests ─────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900)); // century but not div by 400
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn test_float_drift_exists() {
        let proof = prove_float_error(1000);
        assert!(proof.discrepancy > dec!(0));
    }

    #[test]
    fn test_leap_year_accrual() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = compound_interest_calendar_aware(
            dec!(10000), dec!(5), start, end
        );
        assert_eq!(result.year_breakdown[0].days_in_year, 366);
        assert!(result.final_balance > dec!(10500));
    }
}