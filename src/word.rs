use std::fmt::Display;
use std::str::FromStr;

use super::status::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    Ok(Self { data })
  }
}

impl Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s: String = self.data.iter().collect();
    write!(f, "{}", s)
  }
}

impl Word {
  pub fn status(&self, answer: &Word) -> Status {
    let mut answer = answer.clone();
    let mut data = Default::default();

    for i in 0..5 {
      if self.data[i] == answer.data[i] {
        data |= 2 << 2 * i;
        answer.data[i] = '_';
      }
    }

    for i in 0..5 {
      if data >> 2 * i & 3 == 2 {
        continue;
      }

      for j in 0..5 {
        if self.data[i] == answer.data[j] {
          data |= 1 << 2 * i;
          answer.data[j] = '_';
          break;
        }
      }
    }

    Status::new(data)
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
  fn status_1() {
    let w = Word::new(['H', 'E', 'L', 'L', 'O']);
    assert_eq!(w.status(&w), "ggggg".parse().unwrap());
  }

  #[test]
  fn status_2() {
    let w = Word::new(['L', 'A', 'R', 'I', 'O']);
    let a = Word::new(['H', 'E', 'L', 'L', 'O']);
    assert_eq!(w.status(&a), "y___g".parse().unwrap());
  }

  #[test]
  fn status_3() {
    let w = Word::new(['T', 'L', 'E', 'L', 'T']);
    let a = Word::new(['H', 'E', 'L', 'L', 'O']);
    assert_eq!(w.status(&a), "_yyg_".parse().unwrap());
  }

  #[test]
  fn status_4() {
    let w = Word::new(['L', 'L', 'E', 'R', 'T']);
    let a = Word::new(['H', 'E', 'L', 'L', 'O']);
    assert_eq!(w.status(&a), "yyy__".parse().unwrap());
  }

  #[test]
  fn status_5() {
    let w = Word::new(['L', 'L', 'L', 'L', 'L']);
    let a = Word::new(['H', 'E', 'L', 'L', 'O']);
    assert_eq!(w.status(&a), "__gg_".parse().unwrap());
  }

  #[test]
  fn status_6() {
    let w = Word::new(['B', 'U', 'M', 'P', 'H']);
    let a = Word::new(['H', 'U', 'M', 'P', 'H']);
    assert_eq!(w.status(&a), "_gggg".parse().unwrap());
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
    assert_eq!(
      "_____".parse::<Word>(),
      Err("0th char is not an alphabet".into())
    );
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
