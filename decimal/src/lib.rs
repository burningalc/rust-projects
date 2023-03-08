use std::{cmp::Ordering, str::FromStr};

use num_bigint::BigInt;

#[derive(Debug, Clone, Eq)]
pub struct Decimal {
    precision: u32,
    number: BigInt,
    negative: bool,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let negative = input.starts_with('-');
        let precision = input.find('.').map(|i| input.len() - i - 1).unwrap_or(0) as u32;
        let number = BigInt::from_str(&input.replace(['.', '-'], "")).unwrap();

        Some(Decimal {
            precision,
            number,
            negative,
        })
    }

    pub fn extend_precision(&mut self, new_precision: &u32) {
        let precision_diff = new_precision - self.precision;
        self.number = &self.number * BigInt::from(10).pow(precision_diff);
    }

    pub fn shrink_precision(&mut self) {
        // cut all the zero
        let mut number = self.number.to_string();
        let mut zeros = 0;
        while number.ends_with('0') {
            number.pop();
            zeros += 1;
        }
        self.number = BigInt::from_str(&number).unwrap();
        self.precision -= zeros as u32;
    }
}

impl std::ops::Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Decimal) -> Self::Output {
        // extend decimals to common precision
        let mut self_extended = self.clone();
        let mut rhs_extended = rhs.clone();

        let common_precision = self.precision.max(rhs.precision);
        self_extended.extend_precision(&common_precision);
        rhs_extended.extend_precision(&common_precision);
        // add the number
        let (negative, number) = match (self_extended.negative, rhs_extended.negative) {
            (true, true) => (true, self_extended.number + rhs_extended.number),
            (true, false) | (false, true) => {
                let (greater, lesser) = if self_extended.number > rhs_extended.number {
                    (self_extended, rhs_extended)
                } else {
                    (rhs_extended, self_extended)
                };
                (greater.negative, greater.number - lesser.number)
            }
            (false, false) => (false, self_extended.number + rhs_extended.number),
        };

        let mut result = Decimal {
            precision: common_precision,
            number,
            negative,
        };

        result.shrink_precision();
        result
    }
}

impl std::ops::Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Decimal) -> Self::Output {
        // extend decimals to common precision
        let mut self_extended = self.clone();
        let mut rhs_extended = rhs.clone();

        let common_precision = self.precision.max(rhs.precision);
        self_extended.extend_precision(&common_precision);
        rhs_extended.extend_precision(&common_precision);

        // sub the number
        let (negative, number) = match (self_extended.negative, rhs_extended.negative) {
            (true, true) => {
                let (greater, lesser) = if self_extended.number > rhs_extended.number {
                    (&self_extended, &rhs_extended)
                } else {
                    (&rhs_extended, &self_extended)
                };
                (*greater == self_extended, &greater.number - &lesser.number)
            }
            (true, false) => (true, self_extended.number + rhs_extended.number),
            (false, true) => (false, self_extended.number + rhs_extended.number),
            (false, false) => {
                let (greater, lesser) = if self_extended.number > rhs_extended.number {
                    (&self_extended, &rhs_extended)
                } else {
                    (&rhs_extended, &self_extended)
                };
                (*greater == rhs_extended, &greater.number - &lesser.number)
            }
        };

        let mut result = Decimal {
            precision: common_precision,
            number,
            negative,
        };
        result.shrink_precision();
        result
    }
}

impl std::ops::Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Decimal) -> Self::Output {
        // extend decimals to common precision
        let mut self_extended = self.clone();
        let mut rhs_extended = rhs.clone();

        let common_precision = self.precision + rhs.precision;
        self_extended.extend_precision(&common_precision);
        rhs_extended.extend_precision(&common_precision);

        // mul the number
        let (negative, number) = match (self_extended.negative, rhs_extended.negative) {
            (true, true) => (false, self_extended.number * rhs_extended.number),
            (true, false) | (false, true) => (true, self_extended.number * rhs_extended.number),
            (false, false) => (false, self_extended.number * rhs_extended.number),
        };

        let mut result = Decimal {
            precision: common_precision * 2,
            number,
            negative,
        };
        result.shrink_precision();
        result
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        // extend decimals to common precision
        let mut self_extended = self.clone();
        let mut other_extended = other.clone();
        {
            let common_precision = self.precision.max(other.precision);
            self_extended.extend_precision(&common_precision);
            other_extended.extend_precision(&common_precision);
        }

        match (self_extended.negative, other_extended.negative) {
            (false, false) => self_extended.number.cmp(&other_extended.number),
            (true, true) => other_extended.number.cmp(&self_extended.number),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        // extend decimals to common precision
        let mut self_extended = self.clone();
        let mut other_extended = other.clone();
        {
            let common_precision = self.precision.max(other.precision);
            self_extended.extend_precision(&common_precision);
            other_extended.extend_precision(&common_precision);
        }

        self_extended.number == other_extended.number
            && self_extended.negative == other_extended.negative
    }
}
