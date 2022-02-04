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

impl FromStr for Status {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let mut data = 0;

    for i in 0..5 {
      match chars.next().ok_or("not enough chars")? {
        'g' | 'G' => data |= 2 << (2 * i),
        'y' | 'Y' => data |= 1 << (2 * i),
        '_' | ' ' => {}
        _ => return Err(format!("{i}th char is not a status char")),
      };
    }

    if let Some(_) = chars.next() {
      return Err("too much chars".into());
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
      Err("0th char is not a status char".into())
    );
  }

  #[test]
  fn parse_short() {
    assert_eq!("gygy".parse::<Status>(), Err("not enough chars".into()));
  }

  #[test]
  fn parse_long() {
    assert_eq!("ggggyyyy".parse::<Status>(), Err("too much chars".into()));
  }

  #[test]
  fn test_format() {
    let s = Status { data: 0b1000000110 };
    assert_eq!(s.to_string(), "gy__g".to_string());
  }
}
