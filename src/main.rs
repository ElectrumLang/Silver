use std::env;
use std::io;
use std::thread;

struct ElectrumNum {
    nums: Vec<i32>,
    dec: Vec<i32>,
}

struct Code {
    body: String,
}

impl Code {
    pub fn run(&self) -> Any {
        // TODO
    }
}

struct Lambda {
    body: Code,
    arity: i32,
}

impl Lambda {
    pub fn apply(&self /*, TODO find type that is variable arity and multiple types */) -> Any {
        // TODO implement
    }
}

struct Macro {
    body: Lambda,
}

impl Macro {
    pub fn apply(&self, code: Code) -> Code {
        self.body.apply(code)
    }
}

fn main() {
    let ARGV: Vec<_> = env::args().collect(); // named as such for convenience's sake
    // TODO: Everything.
}
