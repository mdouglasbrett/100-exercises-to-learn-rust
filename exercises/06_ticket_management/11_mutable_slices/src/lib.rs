// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut [str]` work? Why or why not?

// @mdouglasbrett - this can't take a &mut String as one of the tests explicitly
// passes a &mut str. I tried using a generic with a trait bound (Into<String>)
// but (sort of obviously String doesn't implement it). Also, we want to mutate
// this in-place, right? So returning a String is also unwanted.
// I originally tried to do this by iterating over the chars but that was unnecessary
pub fn lowercase(s: &mut str) {
    s.make_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
