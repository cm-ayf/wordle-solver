use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
  data: [char; 5],
}

impl FromStr for Word {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.to_ascii_uppercase();
    let mut chars = s.chars();
    let mut data: [char; 5] = [' '; 5];

    for i in 0..5 {
      let c = chars.next().ok_or("not enough chars")?;
      if c < 'A' || c > 'Z' {
        Err(format!("{i}th char is not an alphabet"))?;
      }
      data[i] = c;
    }

    if let Some(_) = chars.next() {
      return Err(format!("too much chars"));
    }

    Ok(Self{ data })
  }
}

impl Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s: String = self.data.iter().collect();
    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  impl Word {
    fn new(data: [char; 5]) -> Self {
      Word { data }
    }
  }

  #[test]
  fn parse_success() {
    let s: Word = "HELLO".parse().unwrap();
    assert_eq!(s, Word::new(['H', 'E', 'L', 'L', 'O']));
  }

  #[test]
  fn parse_from_lower() {
    let s: Word = "World".parse().unwrap();
    assert_eq!(s, Word::new(['W', 'O', 'R', 'L', 'D']));
  }

  #[test]
  fn parse_invalid_char() {
    assert_eq!("_____".parse::<Word>(), Err("0th char is not an alphabet".into()));
  }

  #[test]
  fn parse_short() {
    assert_eq!("long".parse::<Word>(), Err("not enough chars".into()));
  }

  #[test]
  fn parse_long() {
    assert_eq!("shoooort".parse::<Word>(), Err("too much chars".into()));
  }

  #[test]
  fn test_format() {
    let s = Word::new(['W', 'O', 'R', 'L', 'D']);

    assert_eq!(format!("{s}"), "WORLD".to_string());
  }
}