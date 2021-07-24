use std::io;
use std::io::prelude::*;

macro_rules! parse {
    ($x:expr) => {
        $x.unwrap().trim().parse().unwrap()
    };
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(s) = lines.next() {
        println!("{}", run(parse!(s)));
    }
}

fn run(_: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_works() {
        assert_eq!(0, run(0));
    }
}
