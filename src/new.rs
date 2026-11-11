use std::fs::{self, create_dir_all};
use std::path::Path;

pub struct New;

impl New {
    pub fn init(path: &Path) {
        println!("Creating...");

        let folder_name = path.file_name().and_then(|s| s.to_str()).unwrap();
        let cargo_folder_name = to_cargo_name(folder_name);

        // path/src を作る(途中のフォルダもまとめて作る)
        let path_n = path;
        let src = path.join("src");
        if let Err(e) = create_dir_all(&src) {
            panic!("DirCreater error: {:?}", e);
        };

        // src/main.rnx に内容を書く
        let main = r#"f main() {
    p("Hello World");
}
"#;
        if let Err(e) = fs::write(src.join("main.rsx"), main) {
            panic!("Write error: {:?}", e);
        }

        let rustn = format!(
            r#"
[pa]
name = "{}"
version = "0.1.0"
edition = "2024"
[de]
"#,
            cargo_folder_name,
        );
        println!("Adding...");
        if let Err(e) = fs::write(path.join("Rustn.toml"), rustn) {
            panic!("Write error: {:?}", e);
        }

        println!("Complate!..");
    }

    pub fn new(path: &Path, name: &str) {
        let project = path.join(name);
        Self::init(&project) // project フォルダは init の中で作られる
    }
}
fn to_cargo_name(folder_name: &str) -> String {
    let converted = folder_name
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "");

    // 空になった
    if converted.is_empty() {
        panic!("プロジェクト名が空になりました: {:?}", folder_name);
    }

    // 最初の文字が数字
    if converted.chars().next().unwrap().is_numeric() {
        panic!("プロジェクト名が数字で始まっています: {:?}", converted);
    }

    converted
}
