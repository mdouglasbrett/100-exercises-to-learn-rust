// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }
    if v.len() == 1 {
        return v[0];
    }

    // @mdouglasbrett - the result of this is two slices, not vecs
    // so we have to create vecs from them as pointed out in the hint.
    // The bigger picture in all this is that you have to do a lot of std
    // reading to get to solutions here, very tough to intuit the correct answer
    // like I might in JS for example
    let (v1, v2) = v.split_at(v.len() / 2);
    let vec1 = v1.to_vec();
    let vec2 = v2.to_vec();
    let sum1: i32 = vec1.iter().sum();
    // @mdouglasbrett - you have to explicitly move the vec in
    let thread_handle = thread::spawn(move || vec2.iter().sum());
    let sum2: i32 = thread_handle.join().unwrap();
    sum1 + sum2
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
