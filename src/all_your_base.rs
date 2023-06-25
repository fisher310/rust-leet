#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 1 || from_base == 0 {
        return Err(Error::InvalidInputBase);
    }
    if to_base == 1 || to_base == 0 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut num = 0;
    let len = number.len() as u32;
    for (idx, value) in number.into_iter().enumerate() {
        if *value >= from_base {
            return Err(Error::InvalidDigit(*value));
        }
        num += value * from_base.pow(len - idx as u32 - 1);
    }
    if num == 0 {
        return Ok(vec![0]);
    }

    let mut ans = Vec::new();

    while num != 0 {
        ans.push(num % to_base);
        num /= to_base;
    }
    ans.reverse();
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = convert(&[4, 2], 10, 2);

        println!("{:?}", ans);
    }
}
