// #![warn(missing_debug_implementation, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    // <'_> you guess the lifetime
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]

fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

fn it_works() {
    let haystack = "a b c d e ";
    let letters: Vec<_> = StrSplit::new(haystack, "").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, "").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
