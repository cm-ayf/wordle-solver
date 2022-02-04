use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct Status {
  data: u16,
}

impl Status {
  pub fn new(data: u16) -> Self {
    Status { data }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseStatusError {
  NotEnoughChars,
  TooMuchChars,
  NotStatusChar(usize)
}

use ParseStatusError::*;

impl Display for ParseStatusError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      NotEnoughChars => write!(f, "not enough chars"),
      TooMuchChars => write!(f, "too much chars"),
      NotStatusChar(i) => write!(f, "{i}th char is not a status char"),
    }
  }
}

impl Error for ParseStatusError {}

impl FromStr for Status {
  type Err = ParseStatusError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let mut data = 0;

    for i in 0..5 {
      match chars.next().ok_or(NotEnoughChars)? {
        'g' | 'G' => data |= 2 << (2 * i),
        'y' | 'Y' => data |= 1 << (2 * i),
        '_' | ' ' => {}
        _ => return Err(NotStatusChar(i)),
      };
    }

    if let Some(_) = chars.next() {
      return Err(TooMuchChars);
    }

    Ok(Self { data })
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    for i in 0..5 {
      match (self.data >> 2 * i) & 3 {
        2 | 3 => s.push('g'),
        1 => s.push('y'),
        0 => s.push('_'),
        _ => panic!(),
      }
    }
    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_success() {
    let s: Status = "gy_ G".parse().unwrap();
    assert_eq!(s, Status { data: 0b1000000110 });
  }

  #[test]
  fn parse_invalid_char() {
    assert_eq!(
      "hello".parse::<Status>(),
      Err(NotStatusChar(0))
    );
  }

  #[test]
  fn parse_short() {
    assert_eq!("gygy".parse::<Status>(), Err(NotEnoughChars));
  }

  #[test]
  fn parse_long() {
    assert_eq!("ggggyyyy".parse::<Status>(), Err(TooMuchChars));
  }

  #[test]
  fn test_format() {
    let s = Status { data: 0b1000000110 };
    assert_eq!(s.to_string(), "gy__g".to_string());
  }
}
