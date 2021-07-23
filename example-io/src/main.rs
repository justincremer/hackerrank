use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let mut lines = reader.lock().lines();

    macro_rules! parse {
        ($x:expr) => {
            $x.unwrap().trim().parse().unwrap()
        };
    }

    let n = parse!(lines.next().unwrap());
    for line in lines.take(n) {
        println!("{}", run(parse!(line)));
    }
}

fn run(_: i32) -> &'static str {
    "test"
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_works() {
        assert_eq!("test", run(0));
    }
}
