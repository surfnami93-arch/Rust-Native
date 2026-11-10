pub struct Convert {
    count: usize,
    split_code: Vec<&'static str>,
    whatfunction: usize,
    isfunction: bool,
    new_text: &'static str,
    copy_text: &'static str,
    singlecotenshon: char,
    is_single: bool,
    is_dable: bool,
    vec_text: Vec<String>,
    ord_text: char,
    is_brackets: bool,
    next_match: String,
    count_len: usize,
}

impl Convert {
    pub fn convert_rs(text: &'static str) -> &str {
        let mut convert = Self {
            copy_text: text,
            new_text: "",
            split_code: text.split_whitespace().collect::<Vec<&str>>(),
            count: 0,
            is_single: false,
            is_dable: false,
            isfunction: false,
            whatfunction: 0,
            singlecotenshon: "'".chars().next().unwrap(),
            vec_text: Vec::new(),
            ord_text: ' ',
            is_brackets: false,
            next_match: String::new(),
            count_len: 0,
        };
        loop {
            let mut memory_text = String::new();
            match convert.split_code.get_mut(convert.count) {
                Some(p) => {
                    let mut len = p.len();
                    loop {
                        let mut c = p.chars().nth(convert.count_len).unwrap();
                        if convert.next_match != "" && len == 0 {
                            insert_after(p, len - 1, convert.next_match.chars().next().unwrap());
                            convert.next_match = "".to_string();
                            continue;
                        }
                        match c {
                            x if x == convert.singlecotenshon && !convert.is_dable => {
                                convert.is_single = !convert.is_single; // single シングルクオーテーションがあったらダブルが効かないように
                            }
                            d if d == '"' && !convert.is_single => {
                                convert.is_dable = !convert.is_dable; // dable ダブルクオーテーションがあったらシングルが効かないように
                            }
                            f if f == 'f' && convert.whatfunction == 0 => {
                                convert.whatfunction = 1;
                                convert.next_match = "n".to_string();
                            }
                            n if n == 'n'
                                && convert.ord_text == 'f'
                                && convert.whatfunction == 0 =>
                            {
                                convert.whatfunction = 1;
                            }
                            '(' => {
                                convert.is_brackets = true;
                                if convert.whatfunction == 1 {
                                    convert.whatfunction = 2;
                                }
                            }
                            ')' => {
                                convert.is_brackets = false;
                                if convert.whatfunction == 2 {
                                    convert.whatfunction = 3;
                                }
                            }
                            return_value if return_value == '>' && convert.whatfunction == 3 => {
                                c == '-';
                                convert.next_match = ">".to_string();
                            }
                            curly_open if curly_open == '{' && convert.whatfunction == 3 => {
                                convert.whatfunction = 0;
                                convert.isfunction = true;
                            }
                            curly_close if curly_close == '}' => {
                                convert.isfunction = false;
                            }
                            _ => {}
                        }
                        memory_text.push(c);
                        convert.ord_text = c;
                        convert.count_len += 1;
                        if convert.count_len == len {
                            break;
                        }
                    }
                }
                None => break,
            }
            convert.count += 1;
            convert.vec_text.join(&memory_text);
        }
        ""
    }
    pub fn convert_toml(text: &str) -> &str {
        ""
    }
}

fn insert_after(s: &str, n: usize, ch: char) -> String {
    match s.char_indices().nth(n) {
        Some((idx, c)) => {
            let pos = idx + c.len_utf8(); // n番目の文字の直後のバイト位置
            let mut out = String::with_capacity(s.len() + ch.len_utf8());
            out.push_str(&s[..pos]);
            out.push(ch);
            out.push_str(&s[pos..]);
            out
        }
        None => {
            // 文字数が足りなければ末尾に追加
            let mut out = s.to_string();
            out.push(ch);
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_rs() {
        let text = r#"
f main() {
    p("Hello, World");
}
"#;
        println!("{}", Convert::convert_rs(text));
    }
}
