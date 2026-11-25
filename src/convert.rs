#[derive(Clone, Debug)]
pub struct ConvertRs {
    count: usize,
    whatfunction: usize,
    isfunction: bool,
    new_text: String,
    singlecotenshon: char,
    is_single: bool,
    is_dable: bool,
    vec_text: Vec<char>,
    is_brackets: bool,
    count_len: usize,
    is_name: bool,
    is_type: bool,
    curly_count: i32,
    thread_value: String,
    continue_count: i32,
    what: What,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum What {
    Non,
    Thread,
}

impl<'a> ConvertRs {
    pub fn convert_rs(text: &'a str) -> String {
        let mut convert = Self {
            count: 0,
            new_text: String::new(),
            is_single: false,
            is_dable: false,
            isfunction: false,
            whatfunction: 0,
            singlecotenshon: "'".chars().next().unwrap(),
            vec_text: Vec::new(),
            is_brackets: false,
            count_len: 0,
            is_name: false,
            is_type: false,
            curly_count: 0,
            thread_value: String::new(),
            continue_count: 0,
            what: What::Non,
        };
        convert.vec_text = text.chars().collect::<Vec<char>>();
        let copy_vec_text = text.chars().collect::<Vec<char>>();
        for c in convert.vec_text {
            let mut c = c;
            if convert.what == What::Thread && convert.curly_count == 2 {
                convert.thread_value.push(c);
                continue;
            }
            if convert.continue_count != 0 {
                convert.continue_count -= 1;
                continue;
            }
            match c {
                x if x == convert.singlecotenshon && !convert.is_dable => {
                    convert.is_single = !convert.is_single; // single シングルクオーテーションがあったらダブルが効かないように
                }
                d if d == '"' && !convert.is_single => {
                    convert.is_dable = !convert.is_dable; // dable ダブルクオーテーションがあったらシングルが効かないように
                }
                '(' => {
                    convert.is_brackets = true;
                    convert.is_name = false;
                    if convert.whatfunction == 1 {
                        convert.whatfunction = 2;
                    }
                }
                _ if convert.is_name => {}
                ')' => {
                    convert.is_brackets = false;
                    if convert.whatfunction == 2 {
                        convert.whatfunction = 3;
                        convert.is_type = false;
                    }
                }
                f if f == 'f' && convert.whatfunction == 0 => {
                    convert.new_text.push_str("fn");
                    convert.count_len += 1;
                    convert.whatfunction = 1;
                    convert.is_name = true;
                    continue; // 通常の push をスキップ
                }
                colon if colon == ':' && convert.whatfunction == 2 && convert.is_brackets => {
                    convert.is_name = false;
                    convert.is_type = true;
                }
                return_value
                    if return_value == '>'
                        && convert.whatfunction == 3
                        && get_force_str(&copy_vec_text, convert.count - 1) != "-".to_string() =>
                {
                    c = '-';
                    convert.new_text.push_str(">");
                }
                curly_open if curly_open == '{' && convert.whatfunction == 3 => {
                    if convert.whatfunction == 3 {
                        convert.isfunction = true;
                    }
                    convert.whatfunction = 0;
                    match convert.what {
                        What::Thread => match convert.curly_count {
                            0 => {
                                convert.what = What::Thread;
                                convert.curly_count = 1;
                            }
                            1 => {
                                convert.curly_count = 2;
                                convert.new_text.push_str("s.spawn( ||");
                            }
                            _ => {
                                panic!("System Error: curly_count Unexpected value");
                            }
                        },
                        What::Non => {}
                    }
                }
                curly_close if curly_close == '}' => {
                    if convert.isfunction == true {
                        convert.isfunction = false;
                    }
                    if convert.what == What::Thread
                        && 0 < convert.curly_count
                        && convert.curly_count <= 2
                    {
                        match convert.curly_count {
                            2 => {
                                convert.curly_count = 1;
                                Self::convert_rs(convert.thread_value.as_str());
                            }
                            1 => {
                                convert.what = What::Non;
                                convert.curly_count = 0;
                            }
                            _ => {}
                        }
                    }
                }
                print
                    if print == 'p'
                        && !convert.is_name
                        && !convert.isfunction
                        && get_force_str(&copy_vec_text, convert.count + 1) == "(".to_string() =>
                {
                    convert.new_text.push_str("println!");
                    continue;
                }
                thread
                    if thread == 't'
                        && !convert.isfunction
                        && get_force_str(&copy_vec_text, convert.count + 5)
                            == "hreed".to_string() =>
                {
                    convert.what = What::Thread;
                    convert.continue_count = 5;
                    convert.new_text.push_str("thread::scope(|s| ");
                    continue;
                }
                '\n' => {}
                _ => {}
            }
            convert.new_text.push(c);
            convert.count += 1;
        }
        convert.new_text
    }
}

#[derive(Clone, Debug)]
pub struct ConvertToml {
    is_in: bool,
    vec_text: Vec<char>,
    new_text: String,
    is_under: bool,
    is_single: bool,
    is_dable: bool,
    singlecotenshon: char,
    is_value: bool,
    count: usize,
    next_continue: bool,
}
impl ConvertToml {
    pub fn convert_toml(text: &str) -> String {
        let mut convert = Self {
            is_in: false, // []のなか
            vec_text: Vec::new(),
            new_text: String::new(),
            is_under: false, // []のあと
            is_single: false,
            is_dable: false,
            singlecotenshon: "'".chars().next().unwrap(),
            is_value: false,
            count: 0,
            next_continue: false,
        };
        convert.vec_text = text.chars().collect::<Vec<char>>();
        let copy_vec_text = text.chars().collect::<Vec<char>>();
        for c in convert.vec_text {
            if convert.next_continue {
                convert.next_continue = false;
                continue;
            }
            match c {
                x if x == convert.singlecotenshon && !convert.is_dable => {
                    convert.is_single = !convert.is_single; // single シングルクオーテーションがあったらダブルが効かないように
                }
                d if d == '"' && !convert.is_single => {
                    convert.is_dable = !convert.is_dable; // dable ダブルクオーテーションがあったらシングルが効かないように
                }
                '[' => {
                    convert.is_in = true;
                }
                ']' => {
                    convert.is_in = false;
                    convert.is_under = true;
                }
                ecol if ecol == '=' && !convert.is_value => {
                    convert.is_value = true;
                }
                package
                    if package == 'p'
                        && get_force_str_toml(&copy_vec_text, convert.count + 1)
                            == "a".to_string()
                        && convert.is_in
                        && !convert.is_value
                        && !(get_force_str(&copy_vec_text, convert.count + 6)
                            == "ackage".to_string()) =>
                {
                    convert.new_text.push_str("package");
                    convert.next_continue = true;
                    continue;
                }
                dependencies
                    if dependencies == 'd'
                        && get_force_str_toml(&copy_vec_text, convert.count + 1)
                            == "e".to_string()
                        && convert.is_in
                        && !convert.is_value
                        && !(get_force_str_toml(&copy_vec_text, convert.count + 12)
                            == "ependencies".to_string()) =>
                {
                    convert.new_text.push_str("dependencies");
                    convert.next_continue = true;
                    continue;
                }
                workspace_version
                    if workspace_version == 'w'
                        && !convert.is_value
                        && get_force_str_toml(&copy_vec_text, convert.count + 3) == ".ve"
                        && !(get_force_str_toml(&copy_vec_text, convert.count + 16)
                            == "orkspace.version") =>
                {
                    convert.new_text.push_str("workspace.version");
                    continue;
                }

                '\n' => {
                    convert.is_value = false;
                }
                _ => {}
            };
            convert.new_text.push(c);
            convert.count += 1;
        }
        convert.new_text
    }
}

fn get_force_str(s: &Vec<char>, point: usize) -> String {
    match s.get(point + 1) {
        Some(p) => {
            return p.to_string();
        }
        None => {
            return "nagawgawgawgawgawgasfas".to_string();
        }
    }
}

fn get_force_str_toml(s: &Vec<char>, point: usize) -> String {
    match s.get(point) {
        Some(p) => {
            return p.to_string();
        }
        None => {
            return "nagawgawgawgawgawgasfas".to_string();
        }
    }
}
