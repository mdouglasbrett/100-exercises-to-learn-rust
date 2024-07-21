// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.

    // @mdouglasbrett - I would have usually done this naively with recursion
    // but as they mentioned the caching, I did a little research around approaches
    // and sort of liked this one with tabulation (adapted from JS):
    
    // @mdouglasbrett - You can only index with a usize, will check the solution
    // later to see if they did anything like this...
    let n_as_u: usize = n.try_into().unwrap();

    let mut table = vec![0, 1];

    if n < 2 {
        return table[n_as_u];
    }

    for i in 2..=n_as_u {
        table.push(table[i - 1] + table[i - 2]);
    }

    table[n_as_u]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
