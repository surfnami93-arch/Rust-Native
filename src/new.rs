use std::fs::{self, DirBuilder};
use std::path::Path;

pub struct New;

impl New {
    pub fn init(path: &Path) {
        println!("Creating...");

        // path/src を作る(途中のフォルダもまとめて作る)
        let path_n = path;
        let src = path.join("src");
        if let Err(e) = DirBuilder::new().recursive(true).create(&src) {
            panic!("DirBuilder error: {:?}", e);
        };

        // src/main.rnx に内容を書く
        let main = r#"f main
    p("Hello World");
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
            path_n.display(),
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
