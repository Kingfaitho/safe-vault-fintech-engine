# Safe Vault — Precision Financial Engine

Financial software has a silent problem. Most of it uses floating point arithmetic.

Run this in any JavaScript console:

```
0.1 + 0.2 === 0.3  // false
```

That is not a curiosity. In a savings product with a million users, floating point drift translates to regulatory risk, silent losses, and compliance failures. Banks have been fined for less.

Safe Vault is a financial calculation engine built in Rust that eliminates this problem entirely. Every calculation uses fixed-point decimal arithmetic. Every interest calculation is calendar-aware — meaning leap years are handled correctly, not approximated.

This is the kind of thing that matters when money is on the line.

## What it proves

```
Operation: Add 1000 x 0.1
Expected:  100

f64 result:  99.99999999999859
Decimal:     100.0

Discrepancy: 0.0000000000014

In a savings app with millions of users, this is regulatory risk.
```

## What it calculates

- Compound interest with calendar-aware day counting
- Leap year detection and correct day allocation per year
- Float vs decimal comparison with proof of error
- Year-by-year interest breakdown

## Demo

```bash
cargo run -- demo
```

Output:

```
Principal:    £50,000.00
Annual Rate:  4.75%
Period:       2024-01-01 to 2026-01-01

2024 | 365/366 days | Interest: £2,425.34 | LEAP YEAR
2025 | 364/365 days | Interest: £2,542.97

Final Balance:  £54,968.32
Naive result:   £54,989.76
Discrepancy:    £21.44 per user — silent loss at scale
```

## Tests

```
✔ test_leap_year
✔ test_float_drift_exists
✔ test_leap_year_accrual

3 passing
```

## Stack

- Rust
- rust_decimal for fixed-point arithmetic
- chrono for calendar-aware date handling
- clap for CLI interface

## Run it

```bash
git clone https://github.com/Kingfaitho/safe-vault-fintech-engine
cd safe-vault-fintech-engine
cargo run -- demo
cargo test
```

## Roadmap

This is the foundation. What gets built on top of it:

- REST API so any application can call the engine over HTTP
- Mortgage and loan amortization calculations
- Audit trail — every calculation logged, timestamped, and signed
- PDF compliance reports formatted for regulatory submission

The engine is production-ready. The API is next.

---

I come from a product and management background. I understand what fintechs and banks actually need — not just technically correct software, but software that survives audits, scales to millions of users, and gives compliance teams something they can sign off on. That is what this is being built toward.
