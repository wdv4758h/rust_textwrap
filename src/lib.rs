#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;


/// Remove any common leading whitespace from every line in `text`.
pub fn dedent(text: &str) -> String {

    // replace "^[ \t]+$" with "" to clean empty lines
    // findall "(^[ \t]*)(?:[^ \t\n])" to count leading whitespaces
    //
    // (?:exp): non-capturing group
    // (?m): multi-line mode
    //
    // https://doc.rust-lang.org/regex/regex/index.html#grouping-and-flags
    lazy_static! {
        static ref WHITESPACE_ONLY_RE: Regex =
            Regex::new(r"(?m)^[ \t]+$").unwrap();
        static ref LEADING_WHITESPACE_RE: Regex =
            Regex::new(r"(?m)(^[ \t]*)(?:[^ \t\n])").unwrap();
    }

    let text = WHITESPACE_ONLY_RE.replace_all(text, "");
    let mut indents_idx = LEADING_WHITESPACE_RE.find_iter(text.as_str());

    let (start, end) = indents_idx.next().unwrap(); // may fail when nothing
    let mut margin = text[start..end].to_string();

    for (start, end) in indents_idx {
        let indent = text[start..end].to_string();
        if margin.starts_with(indent.as_str()) {
            margin = indent.to_string();
        } else if !indent.starts_with(margin.as_str()) {
            // find longest common whitespaces
            for (i, (x, y)) in margin.to_string()
                                     .chars()
                                     .zip(indent.chars())
                                     .enumerate() {
                if x != y {
                    margin = margin[..i].to_string();
                    break;
                }
            }
        }
    }

    if !margin.is_empty() {
        let mut pattern = r"(?m)^".to_string();
        pattern.push_str(margin.as_str());
        let re = Regex::new(pattern.as_str()).unwrap();
        return re.replace_all(text.as_str(), "");
    }

    text.to_string()
}
