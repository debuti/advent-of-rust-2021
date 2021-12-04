use std::collections::HashSet;

const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

fn main() {
  let data = String::from_utf8_lossy(include_bytes!("input.txt"));
  let data: Vec<&str> = data.split("\n").filter(|x| x.len() > 0).collect();
  let width = data.iter().nth(0).unwrap().len();
  let binreport: Vec<u32> = data
    .iter()
    .map(|x| u32::from_str_radix(x, 2).unwrap())
    .collect();

  let mut gamma: u32 = 0;
  for n in 0..width {
    let mut count = 0;
    for num in &binreport {
      let mask: u32 = 1 << n;
      if num & mask > 0 {
        count += 1;
      }
    }
    if count > (binreport.len() / 2) {
      gamma |= 1 << n;
    }
  }
  let epsilon = !gamma & ((1 << width) - 1);
  debugln!(
    "{}",
    format!(
      "gamma: {:0width$b} epsilon: {:0width$b}",
      gamma,
      epsilon,
      width = width
    )
  );
  println!("1: {}", gamma * epsilon);

  let mut binset: HashSet<u32> = HashSet::from_iter(binreport.clone().into_iter());
  let mut o2 = 0;
  for n in (0..width).rev() {
    let mut count1s = 0;
    for num in &binset {
      let mask: u32 = 1 << n;
      if num & mask > 0 {
        count1s += 1;
      }
    }
    if count1s * 2 >= binset.len() {
      binset = binset.into_iter().filter(|x| (x & 1 << n) > 0).collect();
    } else if count1s * 2 < binset.len() {
      binset = binset.into_iter().filter(|x| (x & 1 << n) == 0).collect();
    }
    if binset.len() == 1 {
      o2 = binset.into_iter().nth(0).unwrap();
      break;
    }
  }

  let mut binset: HashSet<u32> = HashSet::from_iter(binreport.clone().into_iter());
  let mut co2 = 0;
  for n in (0..width).rev() {
    let mut count1s = 0;
    for num in &binset {
      let mask: u32 = 1 << n;
      if num & mask > 0 {
        count1s += 1;
      }
    }
    if count1s * 2 < binset.len() {
      binset = binset.into_iter().filter(|x| (x & 1 << n) > 0).collect();
    } else if count1s * 2 >= binset.len() {
      binset = binset.into_iter().filter(|x| (x & 1 << n) == 0).collect();
    }
    if binset.len() == 1 {
      co2 = binset.into_iter().nth(0).unwrap();
      break;
    }
  }
  debugln!(
    "{}",
    format!("O2: {:0width$b} CO2: {:0width$b}", o2, co2, width = width)
  );
  println!("2: {}", o2 * co2);
}
