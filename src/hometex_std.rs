use thiserror::Error;
pub struct Integer {
    digits: Vec<Digit>,
    negative: bool,
}

pub struct Float {
    digits_pre_dot: Vec<Digit>,
    digits_post_dot: Vec<Digit>,
    repeating: Option<std::ops::RangeInclusive<usize>>,
    negative: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(u8);

impl Digit {
    pub fn new(digit: u8) -> Option<Digit> {
        if digit > 9 {
            None
        } else {
            Some(unsafe { Digit::new_unchecked(digit) })
        }
    }

    pub fn digit(&self) -> u8 {
        self.0
    }

    pub unsafe fn new_unchecked(digit: u8) -> Digit {
        Digit(digit)
    }
}

#[derive(Debug, Error)]
#[error("{0} is not a digit")]
pub struct DigitTryFromCharError(char);

impl TryFrom<char> for Digit {
    type Error = DigitTryFromCharError;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        if !(b'0'..=b'9').contains(&(char as u8)) {
            return Err(DigitTryFromCharError(char));
        }

        Ok(Digit::new(char as u8 - b'0').unwrap())
    }
}

#[derive(Debug, Error)]
#[error("{0} is not in digit range (0 - 9)")]
pub struct DigitTryFromU8Error(u8);

impl TryFrom<u8> for Digit {
    type Error = DigitTryFromU8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 9 {
            return Err(DigitTryFromU8Error(value));
        }

        Ok(Digit::new(value).unwrap())
    }
}
