// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let (v1, v2) = v.split_at(v.len() / 2);
    // @mdouglasbrett - I originally thought that scope did some magic where you
    // never had to call join - but I think that was because the threads in the
    // example never had to be combined
    thread::scope(|scope| {
        let sum1: i32 = scope.spawn(|| v1.iter().sum()).join().unwrap();
        let sum2: i32 = scope.spawn(|| v2.iter().sum()).join().unwrap();
        sum1 + sum2
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
