use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  Gray,
  Yellow,
  Green,
}

impl Default for Color {
  fn default() -> Self {
    Self::Gray
  }
}

impl From<&Color> for char {
  fn from(c: &Color) -> Self {
    match c {
      &Color::Gray => '_',
      &Color::Yellow => 'y',
      &Color::Green => 'g',
    }
  }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Status {
  pub data: [Color; 5],
}

impl FromStr for Status {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let mut data: [Color; 5] = [Color::Green; 5];

    for i in 0..5 {
      data[i] = match chars.next().ok_or("not enough chars")? {
        'g' | 'G' => Color::Green,
        'y' | 'Y' => Color::Yellow,
        '_' => Color::Gray,
        _ => return Err(format!("{i}th char is not a status char")),
      };
    }

    if let Some(_) = chars.next() {
      return Err("too much chars".into());
    }

    Ok(Self{ data })
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    self.data.iter().for_each(|c| s.push(c.into()));
    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_success() {
    let s: Status = "gy_GY".parse().unwrap();
    assert_eq!(s, Status { data: [
      Color::Green,
      Color::Yellow,
      Color::Gray,
      Color::Green,
      Color::Yellow,
    ] });
  }

  #[test]
  fn parse_invalid_char() {
    assert_eq!("hello".parse::<Status>(), Err("0th char is not a status char".into()));
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
    let s = Status { data: [
      Color::Green,
      Color::Yellow,
      Color::Gray,
      Color::Green,
      Color::Yellow,
    ] };

    assert_eq!(format!("{s}"), "gy_gy".to_string());
  }
}
