//! # Pinecone Client
//!
//! `pinecone_client` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, pinecone_client::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
