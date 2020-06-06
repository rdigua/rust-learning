// Replaces all occurrences of the standard ISO 8601 YYYY-MM-DD date pattern with the equivalent American English date with slashes. For example 2013-01-15 becomes 01/15/2013.

// The method Regex::replace_all replaces all occurrences of the whole regex. &str implements the Replacer trait which allows variables like $abcde to refer to corresponding named capture groups (?P<abcde>REGEX) from the search regex. See the replacement string syntax for examples and escaping detail.

extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::borrow::Cow;
use regex::Regex;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX : Regex = Regex::new(
            r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
            ).unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

fn main() {
    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}

