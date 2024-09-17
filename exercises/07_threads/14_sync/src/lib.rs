// Not much to be exercised on `Sync`, just a thing to remember.
fn outro() -> &'static str {
    // @mdouglasbrett - Send is safe to send to another thread
    // Sync can be shared between threads (T is Sync if &T is Send)
    // I am SURE I will at some point need to remind myself of this lol
    "I have a good understanding of Send and Sync!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
