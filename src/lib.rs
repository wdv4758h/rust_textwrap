#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;


/// Remove any common leading whitespace from every line in `text`.
pub fn dedent(text: &str) -> String {
    if text.is_empty() {
        return text.to_owned();
    }

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

    {
        // this will borrow text
        let mut indents_idx = LEADING_WHITESPACE_RE.find_iter(text.as_str());

        let (mut pre_start, mut pre_end) = indents_idx.next().unwrap();
        pre_end = pre_end-1;

        for (start, end) in indents_idx {
            let end = end-1;
            let margin = &text[pre_start..pre_end];  // FIXME, borrow every time
            let indent = &text[start..end];

            if indent.starts_with(margin) {
            } else if margin.starts_with(indent) {
                pre_end = pre_start + (end - start);
            } else if !indent.starts_with(margin) {
                // find longest common whitespaces
                for (i, (x, y)) in margin.chars()
                                         .zip(indent.chars())
                                         .enumerate() {
                    if x != y {
                        pre_end = pre_start + i;
                        break;
                    }
                }
            }
        }

        let margin = &text[pre_start..pre_end];     // final borrow

        if !margin.is_empty() {
            let mut pattern = r"(?m)^".to_string();
            pattern.push_str(margin);
            let re = Regex::new(pattern.as_str()).unwrap();
            return re.replace_all(text.as_str(), "");
        }
    }

    text
}
