/// A lazy man's ```String::from("")``` macro
///
/// I just don't wanna write that man...
///
/// Usage:
/// ```
///    let s = string!("some kind of string");
///
/// ```
#[macro_export]
macro_rules! string {
    ($s: expr) => {
        String::from($s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = string!("it works!");
        assert_eq!(result, "it works!");
    }
}
