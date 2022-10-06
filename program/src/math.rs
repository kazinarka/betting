use crate::error::ContractError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::program_error::ProgramError;
use std::cmp::{max, Ordering};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

const MAX_DECIMALS: u8 = 9;

#[derive(Debug, Copy, Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Number {
    val: u128,
    decimals: u8,
}

impl Number {
    pub const fn new(val: u64, decimals: u8) -> Number {
        Number {
            val: val as u128,
            decimals,
        }
    }

    pub const fn int(val: u64) -> Number {
        Number {
            val: val as u128,
            decimals: 0,
        }
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    #[must_use]
    pub const fn scale(&self, decimals: u8) -> Number {
        if decimals == self.decimals {
            return Number {
                val: self.val,
                decimals,
            };
        }

        let val = if self.decimals > decimals {
            self.val / 10_u128.pow((self.decimals - decimals) as u32)
        } else {
            self.val * 10_u128.pow((decimals - self.decimals) as u32)
        };
        Number { val, decimals }
    }

    pub fn inner(&self) -> Result<(u64, u8), ProgramError> {
        match u64::try_from(self.val) {
            Ok(val) => Ok((val, self.decimals)),
            Err(err) => {
                msg!("Convert with overflow. {}", err);
                Err(ContractError::ConvertWithOverflow.into())
            }
        }
    }

    pub fn value(&self) -> Result<u64, ProgramError> {
        match u64::try_from(self.val) {
            Ok(val) => Ok(val),
            Err(err) => {
                msg!("Convert with overflow. {}", err);
                Err(ContractError::ConvertWithOverflow.into())
            }
        }
    }

    #[allow(clippy::comparison_chain)]
    pub fn mul(&self, other: Self) -> Result<Self, ProgramError> {
        let multiplied = mul(self.val, other.val)?;

        let new_decimals = self.decimals + other.decimals;
        let multiplied_scaled = if new_decimals < MAX_DECIMALS {
            let decimals_underflow = MAX_DECIMALS - new_decimals;
            mul(multiplied, 10_u128.pow(decimals_underflow as u32))?
        } else if new_decimals > MAX_DECIMALS {
            let decimals_overflow = new_decimals - MAX_DECIMALS;
            // scale multiplied down
            div(multiplied, 10_u128.pow(decimals_overflow as u32))?
        } else {
            multiplied
        };

        Ok(Number {
            val: multiplied_scaled,
            decimals: MAX_DECIMALS,
        })
    }

    #[allow(clippy::comparison_chain)]
    fn convert(mut amount: Number, base_decimals: u8, destination_decimals: u8) -> Number {
        if base_decimals > destination_decimals {
            amount = amount
                .div(Number::int(10).pow((base_decimals - destination_decimals) as u32))
                .unwrap();
        } else if base_decimals < destination_decimals {
            amount = amount
                .mul(Number::int(10).pow((destination_decimals - base_decimals) as u32))
                .unwrap();
        }

        amount
    }

    pub fn convert_to_18(amount: Number, base_decimals: u8) -> Number {
        if base_decimals == 18 {
            return amount;
        }
        Number::convert(amount, base_decimals, 18)
    }

    pub fn convert_from_18(amount: Number, destination_decimals: u8) -> Number {
        if destination_decimals == 18 {
            return amount;
        }
        Number::convert(amount, 18, destination_decimals)
    }

    pub fn pow(&self, exp: u32) -> Self {
        Number::int(self.val.pow(exp) as u64)
    }

    pub fn add(&self, other: Self) -> Result<Self, ProgramError> {
        let max_dec = max(self.decimals, other.decimals);

        let num1_scaled = mul(self.val, pow_10(max_dec - self.decimals))?;
        let num2_scaled = mul(other.val, pow_10(max_dec - other.decimals))?;

        Ok(Self {
            val: add(num1_scaled, num2_scaled)?,
            decimals: max_dec,
        })
    }

    #[allow(clippy::comparison_chain)]
    pub fn div(&self, other: Self) -> Result<Self, ProgramError> {
        let num1_scaling_factor = pow_10(MAX_DECIMALS - self.decimals);
        let num1_scaled = mul(self.val, num1_scaling_factor)?;

        let num1_scaled_with_overflow = mul(num1_scaled, pow_10(MAX_DECIMALS))?;

        let num2_scaling_factor = pow_10(MAX_DECIMALS - other.decimals);
        let num2_scaled = mul(other.val, num2_scaling_factor)?;
        let division = div(num1_scaled_with_overflow, num2_scaled)?;
        Ok(Self {
            val: division,
            decimals: MAX_DECIMALS,
        })
    }

    pub fn sub(&self, other: Self) -> Result<Self, ProgramError> {
        let max_dec = max(self.decimals, other.decimals);

        let num1_scaled = mul(self.val, pow_10(max_dec - self.decimals))?;
        let num2_scaled = mul(other.val, pow_10(max_dec - other.decimals))?;

        Ok(Self {
            val: sub(num1_scaled, num2_scaled)?,
            decimals: max_dec,
        })
    }
}

impl Eq for Number {}

impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.decimals == other.decimals {
            self.val.eq(&other.val)
        } else {
            let num_1 = self.scale(MAX_DECIMALS);
            let num_2 = other.scale(MAX_DECIMALS);
            num_1.val.eq(&num_2.val)
        }
    }
}

impl PartialOrd<Self> for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.decimals == other.decimals {
            self.val.partial_cmp(&other.val)
        } else {
            let num_1 = self.scale(MAX_DECIMALS);
            let num_2 = other.scale(MAX_DECIMALS);
            num_1.val.partial_cmp(&num_2.val)
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.decimals == other.decimals {
            self.val.cmp(&other.val)
        } else {
            let num_1 = self.scale(MAX_DECIMALS);
            let num_2 = other.scale(MAX_DECIMALS);
            num_1.val.cmp(&num_2.val)
        }
    }
}

fn mul(a: u128, b: u128) -> Result<u128, ProgramError> {
    if let Some(val) = a.checked_mul(b) {
        Ok(val)
    } else {
        msg!("Mul with overflow");
        Err(ContractError::OperationWithOverflow.into())
    }
}

fn div(a: u128, b: u128) -> Result<u128, ProgramError> {
    if let Some(val) = a.checked_div(b) {
        Ok(val)
    } else {
        msg!("Div with overflow");
        Err(ContractError::OperationWithOverflow.into())
    }
}

fn add(a: u128, b: u128) -> Result<u128, ProgramError> {
    if let Some(val) = a.checked_add(b) {
        Ok(val)
    } else {
        msg!("Add with overflow");
        Err(ContractError::OperationWithOverflow.into())
    }
}

fn sub(a: u128, b: u128) -> Result<u128, ProgramError> {
    if let Some(val) = a.checked_sub(b) {
        Ok(val)
    } else {
        msg!("Sub with overflow");
        Err(ContractError::OperationWithOverflow.into())
    }
}

const fn pow_10(exp: u8) -> u128 {
    10_u128.pow(exp as u32)
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.val as f64 / 10_usize.pow(self.decimals as u32) as f64
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Number;
    use std::cmp::Ordering;

    #[test]
    pub fn test_view() {
        assert_eq!(&Number::new(10_100000, 6).to_string(), "10.1");
        assert_eq!(&Number::new(1, 0).to_string(), "1");
        assert_eq!(&Number::new(3_141592653, 9).to_string(), "3.141592653");
    }

    #[test]
    pub fn test_convert() {
        assert_eq!(&Number::new(10_100130, 6).scale(5).to_string(), "10.10013");
        assert_eq!(&Number::new(10_100133, 6).scale(6).to_string(), "10.100133");
        assert_eq!(&Number::new(10_100000, 6).scale(1).to_string(), "10.1");
        assert_eq!(&Number::new(10_100000, 6).scale(0).to_string(), "10");

        assert_eq!(&Number::new(1, 0).scale(5).to_string(), "1");
        assert_eq!(
            &Number::new(3_141592653, 9).scale(6).to_string(),
            "3.141592"
        );
    }

    #[test]
    pub fn test_mul() {
        test((45, 0), (19, 0), "855");
        test((2_0, 1), (3, 0), "6");
        test((2_000000000, 9), (3_000000000, 9), "6");
        test((1_000000000, 9), (2, 0), "2");
        test((1_00000000, 8), (2_000, 3), "2");

        test((3_141592653, 9), (2_000, 3), "6.283185306");

        fn test(a: (u64, u8), b: (u64, u8), res: &str) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(&a.mul(b).unwrap().to_string(), res);
        }
    }

    #[test]
    pub fn test_div() {
        test((6, 0), (2, 0), "3");
        test((4_000000000, 9), (2, 0), "2");
        test((3, 0), (2, 0), "1.5");
        test((15, 1), (2, 1), "7.5");
        test((62_000000, 6), (100_000000000, 9), "0.62");
        test((62_000000, 6), (100_000000000, 9), "0.62");
        test((6_283185306, 9), (2_000, 3), "3.141592653");

        fn test(a: (u64, u8), b: (u64, u8), res: &str) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(&a.div(b).unwrap().to_string(), res);
        }
    }

    #[test]
    pub fn test_add() {
        test((1, 0), (2, 0), "3");
        test((1_000000000, 9), (2_000000000, 9), "3");
        test((1_000, 3), (2_000000000, 9), "3");

        fn test(a: (u64, u8), b: (u64, u8), res: &str) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(&a.add(b).unwrap().to_string(), res);
        }
    }

    #[test]
    pub fn test_sub() {
        test((3, 0), (1, 0), "2");
        test((3_000000000, 9), (1_000000, 6), "2");
        test((3_000, 3), (2_00000, 5), "1");

        fn test(a: (u64, u8), b: (u64, u8), res: &str) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(&a.sub(b).unwrap().to_string(), res);
        }
    }

    #[test]
    pub fn test_eq() {
        test((1, 0), (1, 0), true);
        test((10, 1), (1, 0), true);
        test((10, 1), (1_000000000, 9), true);
        test((10, 1), (1_000000001, 9), false);

        fn test(a: (u64, u8), b: (u64, u8), res: bool) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(a.eq(&b), res);
        }
    }

    #[test]
    pub fn test_cmp() {
        test((2, 0), (1, 0), Ordering::Greater);
        test((2, 0), (3, 0), Ordering::Less);
        test((3, 0), (3, 0), Ordering::Equal);

        test((2_00, 2), (1, 0), Ordering::Greater);
        test((2_00, 2), (3, 0), Ordering::Less);
        test((3_00, 2), (3, 0), Ordering::Equal);

        fn test(a: (u64, u8), b: (u64, u8), res: Ordering) {
            let a = Number::new(a.0, a.1);
            let b = Number::new(b.0, b.1);
            assert_eq!(a.cmp(&b), res);
        }
    }
}