use std::io;
use std::io::prelude::*;

struct Field {
    inner: Vec<i64>,
}

impl Field {
    pub fn new(inner: Vec<i64>) -> Self {
        Self { inner }
    }

    pub fn max_sum(&self) -> i64 {
        self.sum_hour_glasses().iter().cloned().fold(0, i64::max)
    }

    fn sum_hour_glasses(&self) -> Vec<i64> {
        let hour_glasses: Vec<Vec<i64>> = self.get_hour_glasses();
        hour_glasses.into_iter().map(|i| i.iter().sum()).collect()
    }

    fn get_hour_glasses(&self) -> Vec<Vec<i64>> {
        let mut res: Vec<Vec<i64>> = vec![vec![]];
        for i in 0..4 {
            for j in 0..4 {
                res.push(self.point_to_hourglass(i * j + i));
            }
        }
        res
    }

    fn point_to_hourglass(&self, i: usize) -> Vec<i64> {
        let key = [i, i + 1, i + 2, i + 7, i + 12, i + 13, i + 14];
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if key.contains(&i) { Some(v) } else { None })
            .collect::<Vec<i64>>()
    }
}

fn parse_line(i: String) -> Vec<i64> {
    i.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input: Vec<i64> = (0..6)
        .map(|_| parse_line(lines.next().unwrap().expect("malformed input stream")))
        .fold(vec![], |mut acc, mut i| {
            acc.append(&mut i);
            acc
        });
    let field = Field::new(input);

    println!("{}", field.max_sum());
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {}
// }
