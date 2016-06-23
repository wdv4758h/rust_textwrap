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


fn trim_not_empty(text: &str) -> bool {
    !text.trim().is_empty()
}


/// Adds 'prefix' to the beginning of selected lines in 'text'.
pub fn indent(text: &str,
              prefix: &str,
              predicate: Option<&Fn(&str)->bool>) -> String {

    // predicate examples:
    //
    //     None
    //
    //     Some(&str::is_empty)
    //
    //     fn f(x: &str) -> bool { false };
    //     Some(&f)
    //
    //     Some(&|x| false)

    let func_default = &trim_not_empty;  // borrow here to live long enough ...

    let func = match predicate {
        Some(f) => f,
        None => func_default,
    };

    // .join
    // https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html
    //
    // FIXME, collect to String directly ? collect::<String>
    text.lines()
        .map(|line|
                if func(line) {
                    format!("{}{}", prefix, line)
                } else {
                    line.to_string()
                })
        .collect::<Vec<_>>()
        .join("\n")
}
