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
        let n = lines
            .next()
            .expect("Must provide a valid i64 as the second argument");
        println!(
            "{}",
            run(
                s.expect("Must provide a valid UTF-8 string").as_str(),
                parse!(n)
            )
        );
    }
}

fn run(s: &str, n: i64) -> i64 {
    let s = String::from(s);
    let s_len = s.len() as i64;
    if s_len == 0 {
        return 0;
    } else if s_len == s.matches("a").count() as i64 {
        return n;
    } else {
        let mut res = (0..n % s_len).fold(0, |acc, i| {
            acc + (s.chars().nth(i as usize).unwrap() == 'a') as i64
        });
        if n > s_len {
            res += (n / s_len) * (s.matches("a").count() as i64);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_works() {
        assert_eq!(6, run("ababa", 10));
        assert_eq!(2, run("ababa", 4));
    }

    #[test]
    fn all_a() {
        assert_eq!(10, run("a", 10));
        assert_eq!(10, run("aaa", 10));
    }

    #[test]
    fn empty() {
        assert_eq!(0, run("", 10));
    }
}
