use std::ascii::AsciiExt;

pub fn is_keyword(txt: &str) -> bool {
    match txt {
        "abstract" | "alignof" | "as" | "become" | "box" |
        "break" | "const" | "continue" |  "crate" | "do" |
        "else" | "enum" | "extern" | "false" | "final" |
        "fn" | "for" | "if" | "impl" | "in" |
        "let" | "loop" | "macro" | "match" | "mod" |
        "move" | "mut" | "offsetof" | "override" | "priv" |
        "proc" | "pub" | "pure" | "ref" | "return" |
        "Self" | "self" | "sizeof" | "static" | "struct" |
        "super" | "trait" | "true" | "type" | "typeof" |
        "unsafe" | "unsized" | "use" | "virtual" | "where" |
        "while" | "yield" => true,
        _ => false
    }
}

pub fn snake_to_camel(input: &str) -> String {
    input.split('_').flat_map(|s| {
        let mut first = true;
        s.chars().map(move |c| {
            if first {
                first = false;
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
    }).collect()
}