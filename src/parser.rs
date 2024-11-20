use std::path::Path;
use std::fs::read_to_string;

// Atomic Types
union number {
    integer : i64,
    double : f64
}

struct Number {
    t : bool,
    v : number
}


// Airthmetic
struct Add {
    a : Number,
    b : Number
}

struct Mult {
    a : Number,
    b : Number
}

struct Div {
    a : Number,
    b : Number
}

struct Pow {
    a : Number,
    b : Number
}

struct Mod {
    a : Number,
    b : Number
}


// Mathematical Validity Checks
impl Div {
    pub fn check_div_by_zero(&self) -> bool {
        if self.b.t {
            unsafe {
                return self.b.v.double != 0.0;
            }
        } else {
            unsafe {
                return self.b.v.integer != 0;
            }
        }
    }
}

impl Mod {
    pub fn check_mod_by_zero (&self) -> bool {
        if self.b.t {
            unsafe {
                return self.b.v.double != 0.0;
            }
        } else {
            unsafe {
                return self.b.v.integer != 0;
            }
        } 
    }
}

pub fn parse_num(s : &str) -> Number {
    if s.contains('.') {
        Number{t: true, v: number{ double: s.parse::<f64>().unwrap()}}
    } else {
        Number{t: false, v: number{ integer: s.parse::<i64>().unwrap()}}
    }
}

pub fn parse(p : &str) {
    let x = read_to_string(p).unwrap();
    let lines : Vec<&str> = x.split('\n').collect();
    let lines : Vec<&str> = lines.iter().map(|s:&&str| s.split(';').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>().into_iter().flatten().collect();
    let strtoks : Vec<Vec<&str>> = lines.iter().map(|s:&&str| s.split_whitespace().collect::<Vec<&str>>()).collect();
    let stack = Vec::new();
    for line in strtoks {
        for i in 0..line.len() {
            stack.append(match strtoks[i] {
                "+" => ,
                "-" => ,
                "*" => ,
                "/" => ,
                "**" => ,
                "%" => ,
                n => parse_num(n),
            });
        }
    }
}