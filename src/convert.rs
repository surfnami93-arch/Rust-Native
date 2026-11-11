#[derive(Clone, Debug)]
pub struct Convert_rs {
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
}

impl<'a> Convert_rs {
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
        };
        convert.vec_text = text.chars().collect::<Vec<char>>();
        let copy_vec_text = text.chars().collect::<Vec<char>>();
        for c in convert.vec_text {
            let mut c = c;
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
                    convert.whatfunction = 0;
                    convert.isfunction = true;
                }
                curly_close if curly_close == '}' => {
                    convert.isfunction = false;
                }
                print
                    if print == 'p'
                        && !convert.is_name
                        && get_force_str(&copy_vec_text, convert.count + 1) == "(".to_string() =>
                {
                    convert.new_text.push_str("println!");
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
pub struct Convert_toml {
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

impl Convert_toml {
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
                        && get_force_str_toml(&copy_vec_text, convert.count + 2)
                            == "e".to_string()
                        && convert.is_in
                        && !convert.is_value
                        && !(get_force_str_toml(&copy_vec_text, convert.count + 11)
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
