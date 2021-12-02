use regex::Regex;

#[derive(Debug)]
enum Cmd {
  Up(i32),
  Down(i32),
  Forward(i32),
}

impl Cmd {
  fn apply(&self, pos: &mut (i32, i32)) {
    *pos = match self {
      Cmd::Up(v)      => (pos.0    , pos.1 - v),
      Cmd::Down(v)    => (pos.0    , pos.1 + v),
      Cmd::Forward(v) => (pos.0 + v, pos.1    ),
    };
  }
  fn apply_w_aim(&self, pos: &mut (i32, i32, i32)) {
    *pos = match self {
      Cmd::Up(v)      => (pos.0    , pos.1            , pos.2 - v),
      Cmd::Down(v)    => (pos.0    , pos.1            , pos.2 + v),
      Cmd::Forward(v) => (pos.0 + v, pos.1 + (pos.2*v), pos.2    ),
    };                                                  
  } 
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("input.txt"));
    let cmds : Vec<Cmd> = data.split("\n").filter(|x| x.len() > 0)
                          .map(|x| {
                            match Regex::new(r"^(.*) (.*)$").unwrap().captures(x) {
                              Some(x) => {
                                let opcode = x.get(1).unwrap().as_str();
                                let value  = x.get(2).unwrap().as_str().parse::<i32>().unwrap();

                                match opcode {
                                  "up" => Cmd::Up(value),
                                  "down" => Cmd::Down(value),
                                  "forward" => Cmd::Forward(value),
                                  _ => unreachable!(),
                                }
                              },
                              None => {
                                panic!("Parsing failed in {}", x);
                              },
                            }
                          }).collect();

    let mut pos : (i32, i32) = (0, 0);
    for cmd in &cmds {
      cmd.apply(&mut pos);
    }
    println!("1: {}", pos.0*pos.1);

    let mut pos : (i32, i32, i32) = (0, 0, 0);
    for cmd in &cmds {    
      cmd.apply_w_aim(&mut pos);
    }
    println!("2: {}", pos.0*pos.1);
    
}
