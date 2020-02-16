/*
** src/types/password.rs
*/

use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct Password {
    digits: [u8; 6],
}

impl Password {
    /// check if the password contains any adjacent repeated digits
    pub fn contains_repeat(&self) -> bool {
        for dn in 0..5 {
            if self.digits[dn] == self.digits[dn + 1] {
                return true;
            }
        }

        false
    }

    /// check if the password contains exactly 2 adjacent repeated digits
    pub fn contains_2repeat(&self) -> bool {
        let mut dcnt = [0; 10];

        for &d in self.digits.iter() {
            dcnt[d as usize] += 1;
        }

        dcnt.into_iter().find(|&&dcnt| dcnt == 2).is_some()
    }

    // increment with wrap-around
    fn increment(mut self) -> Self{
        for dn in (0..6).rev() {
            self.digits[dn] += 1;
            // carry
            if self.digits[dn] == 10 {
                self.digits[dn] = 0;
            } else {
                break;
            }
        }

        self
    }

    // extend the maximum non-zero digit through all digits subsequent to it
    fn digit_extend(&mut self) {
        let max_digit = *self.digits.iter().max().unwrap();

        // find index of max digit
        let (idx, _) = self.digits.iter()
            .enumerate()
            .find(|(_, &d)| d == max_digit).unwrap();

        for i in idx..6 {
            self.digits[i] = max_digit;
        }
    }

    // generate an increasing sequence of passwords from the 6-digit run of
    // the lower-bound digit to the 6-digit run of the upper-bound digit
    // only maintains the condition that digits must be increasing, does NOT
    // maintain that there are adjacent repeated digits
    fn generate_incr_seq(lb_digit: u8, ub_digit: u8) -> Vec<Self> {
        let mut gen = vec![];

        // start with the 6-digit run of the lower-bound digit
        gen.push(Password::from([lb_digit; 6]));
        // generate the 6-digit run of the upper-bound digit to use as an
        // upper-bound for the generator loop
        let ub = Password::from([ub_digit; 6]);

        while gen[gen.len() - 1] < ub {
            let mut p = gen[gen.len() - 1].clone().increment();

            // if the password rolled over, extend the last non-zero digit
            if p.digits[5] == 0 {
                p.digit_extend();
            }

            gen.push(p);
        }
        gen.push(ub);

        gen
    }

    /// generate passwords in the provided range (inclusive)
    /// this will maintain the condition that all digits are increasing but
    /// will NOT maintain the repeat digit conditions
    pub fn generate_in_range(lower: i64, upper: i64) -> Vec<Self> {
        // for ease of implementation, expand the bounds to the nearest thousand
        let lower_digit = (lower / 100000) as u8;
        let upper_digit = ((upper / 100000) + 1) as u8;

        // generate an increasing sequence maintaining the condition that all
        // digits must be increasing
        let gen = Password::generate_incr_seq(lower_digit, upper_digit);

        // trim down the generated set for the precise given bounds
        let p_lo = Password::from(lower);
        let p_hi = Password::from(upper);
        gen.into_iter()
            .filter(|p| p >= &p_lo && p <= &p_hi)
            .collect()
    }
}

impl From<[u8; 6]> for Password {
    fn from(digits: [u8; 6]) -> Self {
        Self { digits }
    }
}

impl From<i64> for Password {
    fn from(mut n: i64) -> Self {
        let mut digits = [0; 6];

        for d in digits.iter_mut().rev() {
            *d = (n % 10) as u8;
            n /= 10;
        }

        Self { digits }
    }
}

impl Ord for Password {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare passwords digit-by-digit
        for (s_dig, o_dig) in self.digits.iter().zip(other.digits.iter()) {
            match s_dig.cmp(o_dig) {
                Ordering::Equal => {},
                res @ _ => return res,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Password {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password: {}{}{}{}{}{}",
               self.digits[0], self.digits[1], self.digits[2],
               self.digits[3], self.digits[4], self.digits[5])
    }
}
