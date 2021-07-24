use std::io;
use std::io::prelude::*;

const SAFE: bool = false;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(_) = lines.next() {
        let stream = lines.next().expect("invalid stream").unwrap();
        let input = stream
            .chars()
            .filter_map(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                _ => None,
            })
            .collect::<Vec<bool>>();

        println!("{}", shortest_path(0, input));
    }
}

fn shortest_path(acc: u64, tail: Vec<bool>) -> u64 {
    // println!("{:?} -> {:?}", acc, tail);
    match tail.len() {
        0 => acc,
        1 | 2 | 3 => acc + 1,
        _ => shortest_path(
            acc + 1,
            if tail[2] == SAFE {
                slice_to_vec(&tail[2..])
            } else {
                slice_to_vec(&tail[1..])
            },
        ),
    }
}

fn slice_to_vec<T>(i: &[T]) -> Vec<T>
where
    T: Clone,
{
    i.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::shortest_path;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            shortest_path(0, vec![false, false, false, true, false, false])
        );
        assert_eq!(3, shortest_path(0, vec![false, false, true, false, false]));
        assert_eq!(
            4,
            shortest_path(0, vec![false, false, true, false, false, true, false])
        );
    }
}
