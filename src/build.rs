use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Build {
    path: PathBuf,
    path_ord: PathBuf,
    contents: String,
}

impl Build {
    pub fn build(path: &Path, build_type: &'static str) -> io::Result<()> {
        let project_pass = path;
        let project_name_m = project_pass.file_name();
        let mut project_name = OsStr::new("");
        match project_name_m {
            Some(name) => project_name = name,
            None => project_name = OsStr::new("unknown"),
        }
        let file_set: Vec<PathBuf> = WalkDir::new(path.to_path_buf())
            .into_iter()
            .filter_entry(|e| {
                // targetフォルダには降りない
                let relative = e
                    .path()
                    .strip_prefix(path.to_path_buf())
                    .unwrap_or(e.path());
                !relative.starts_with("target")
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| {
                e.path()
                    .strip_prefix(path.to_path_buf())
                    .ok()
                    .map(|p| p.to_path_buf())
            })
            .collect();
        let mut file_set_all: Vec<Self> = Vec::new();
        let _: () = file_set
            .iter()
            .map(|s| {
                file_set_all.push(Self {
                    path: project_pass
                        .to_path_buf()
                        .join("target")
                        .join("cargo")
                        .join(project_name)
                        .join(s),
                    path_ord: project_pass.to_path_buf().join(s),
                    contents: String::new(),
                })
            })
            .collect();
        file_set_all.retain(|p| {
            let ext = p.path.extension().and_then(|e| e.to_str());
            ext == Some("rsx") || ext == Some("toml")
        });

        for data in &mut file_set_all {
            if data.path.extension().and_then(|e| e.to_str()) == Some("rsx") {
                let file_name_for = &data.path_ord.file_name().unwrap();
                println!("Transforming rust... ==> {}", file_name_for.display());
                let content = fs::read_to_string(data.path_ord.clone())?;
                data.contents = crate::convert::Convert_rs::convert_rs(&content);
                if data.path.extension().and_then(|e| e.to_str()) == Some("rsx") {
                    data.path.set_extension("rs");
                }
            } else {
                let file_name_for = &data.path_ord.file_name().unwrap();
                println!("Transforming toml... ==> {}", file_name_for.display());
                let content = fs::read_to_string(data.path_ord.clone())?;
                data.contents = crate::convert::Convert_toml::convert_toml(&content);
                if let Some(stem) = data.path.file_stem().and_then(|s| s.to_str()) {
                    if stem == "Rustn" {
                        let ext = data.path.extension().map(|e| e.to_os_string());
                        data.path.set_file_name("Cargo");
                        if let Some(ext) = ext {
                            data.path.set_extension(ext);
                        }
                    }
                }
            }
        }
        println!("Transforming OK");
        for data in file_set_all {
            println!(
                "Adding... => {}",
                data.path_ord.file_name().unwrap_or_default().display()
            );
            // data.path = pathの完成形
            if let Some(parent) = data.path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&data.path, data.contents)?;
        }
        println!("Adding OK");
        let debug_pass: PathBuf = project_pass.join("target").join("exe").join("debug.exe"); // コピー先
        let project_cargo: PathBuf = project_pass.join("target").join("cargo").join(project_name); // C:\\Users\\surfn\\rustn\\test\\target\\cargo // C:\\Users\\surfn\\rustn\\testはRustNativeProjectpass
                                                                                                   // execute
        cargo_excuting(project_cargo.to_path_buf(), build_type)?;
        // exe移行
        fs::create_dir_all(project_pass.join("target").join("exe"))?;
        let dst = project_cargo // コピー元
            .join("target")
            .join("debug")
            .join(format!("{}.exe", project_name.display()));
        fs::copy(&dst, &debug_pass)?;
        Ok(())
    }
}

fn cargo_excuting(path: PathBuf, type_str: &'static str) -> io::Result<()> {
    let cargo_path = crate::utils::command_utils::cargo_search_command()?;
    assert!(path.is_absolute(), "pathが相対パスになっている");
    assert!(path.exists(), "pathが存在しない: {:?}", path);
    assert!(path.is_dir(), "pathがディレクトリでない: {:?}", path);
    let status = match type_str {
        "b" => Command::new(&cargo_path)
            .arg("build")
            .current_dir(&path)
            .status()?,
        "r" => Command::new(&cargo_path)
            .arg("run")
            .current_dir(&path)
            .status()?,
        "re" => Command::new(&cargo_path)
            .args(["build", "--release"])
            .current_dir(&path)
            .status()?,
        _ => panic!("予想されないエラーです"),
    };
    if !status.success() {
        panic!("");
    }
    Ok(())
}
