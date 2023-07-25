
mod escaper;

pub use escaper::*;

mod unescaper;

pub use unescaper::*;

mod skip_chars;

pub use skip_chars::*; 


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
