use std::env;
mod build;
mod new;
mod utils;
use build::Build;
mod convert;
use new::New;

const LICENCE: &str = r#"MIT License

Copyright (c) [2026] [surfnami-arch]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#;

const HELP: &str = r#"run:                                そのプログラムをビルドして実行します。
build:                              そのプログラムをビルドします。
release:                            そのプログラムを高度なレベルでビルドします。
init:                               そのプロジェクトを初期化します。
new [project name]:                 指定のプロジェクト名のプロジェクトフォルダを作成します。
-v or -version or --v or --version: バージョンを表示します。
licence or -licence:                ライセンスを表示します。
-help or help:                      ヘルプを表示します。"#;

fn main() {
    let cmds: Vec<String> = env::args().collect();
    let path = utils::dir();
    match cmds.get(1).map(|s| s.as_str()) {
        Some("run") | Some("r") => match Build::build(&path.as_path(), "r") {
            Ok(()) => {}
            Err(e) => println!("Err: {e}"),
        },
        Some("build") | Some("b") => match Build::build(&path.as_path(), "b") {
            Ok(()) => println!("Building OK!"),
            Err(e) => println!("Err: {e}"),
        },
        Some("new") => match check(&cmds) {
            Some(r) => New::new(&path, r),
            None => panic!("ファイル名を指定してください"),
        },
        Some("init") => New::init(&path),
        Some("-version") | Some("-v") | Some("--version") | Some("--v") => {
            println!("v0.1.0");
        }
        Some("release") => match Build::build(&path.as_path(), "re") {
            Ok(()) => println!("Release OK!"),
            Err(e) => println!("Err: {e}"),
        },
        Some("-licence") | Some("licence") => {
            println!("{LICENCE}");
        }
        Some("-help") | Some("help") => {
            println!("{HELP}");
        }
        None => {
            println!("Hello!");
        }
        Some(unknown) => {
            println!("Unknown command: {}", unknown);
        }
    }
}
fn check(cmds: &[String]) -> Option<&str> {
    match cmds.get(2).map(|s| s.as_str()) {
        Some(p) => Some(p),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_rs() {
        let text = r#"f main
    p("Hello World");
"#;
        dbg!(crate::convert::Convert_rs::convert_rs(text));
    }
    #[test]
    fn test_build() {
        match Build::build(utils::dir().as_path(), "b") {
            Ok(()) => println!("Building OK!"),
            Err(e) => println!("Err: {e}"),
        };
    }
}
