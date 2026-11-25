use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let la = input("Please select a language. English: E | Japanese: J");
    match la.as_str() {
        "E" => english_main(),
        "J" => japanese_main(),
        _ => panic!("This is an unrecognized language."),
    }
}

fn japanese_main() {
    println!("インストールしていただきありがとうございます");
    println!("\n   ようこそ Rust Nativeへ");
    println!("\nインストール...");
    let path = r"C:\Program Files\Rust Native";
    if let Err(e) = install(Some(path), "J") {
        panic!("ダウンロード エラー: {}", e);
    }
    println!("Rust Nativeを開くには rustnと入力して下さい");
}

fn english_main() {
    println!("Thank you for install");
    println!("Welcome Rust Native");
    println!("\ninstall...");
    let path = r"Rust Native";
    if let Err(e) = install(Some(path), "E") {
        panic!("Dawnload Err: {}", e);
    }
    println!("To open “Rust Native,” type “rustn.” ");
}

fn input(value: &str) -> String {
    let mut input = String::new();

    print!("{}", value);
    io::stdout().flush().unwrap(); // ← これを追加

    io::stdin().read_line(&mut input).expect("入力エラー");

    input.trim().to_string()
}
fn relaunch_as_admin() {
    let exe = std::env::current_exe().unwrap();
    Command::new("powershell")
        .args([
            "-Command",
            "Start-Process",
            &exe.to_string_lossy(),
            "-Verb",
            "RunAs",
        ])
        .spawn()
        .unwrap();
    std::process::exit(0);
}

fn is_elevated() -> bool {
    Command::new("net")
        .args(["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn download(la: &str) -> io::Result<Vec<u8>> {
    let url = "http://rustnative.porikun.net/rustn-windows-x64-setup.exe";
    let response = ureq::get(url)
        .call()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut buf = Vec::new();
    response.into_reader().read_to_end(&mut buf)?;

    if la == "E" {
        println!("Sucess dawnload");
    } else {
        println!("ダウンロード完了");
    }
    Ok(buf)
}

pub fn install(path: Option<&str>, la: &str) -> io::Result<()> {
    if !is_elevated() {
        println!("管理者権限が必要です。再起動します...");
        relaunch_as_admin();
        return Ok(());
    }

    let data = download(la)?;

    let install_dir = match path {
        Some(p) => {
            let program_files =
                std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
            PathBuf::from(program_files).join(p)
        }
        None => PathBuf::from(r"C:\Program Files\Rust Native"),
    };

    fs::create_dir_all(&install_dir)?;
    let dst = install_dir.join("rustn.exe");
    fs::write(&dst, &data)?;

    // PATH環境変数に追加
    Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('Path', $env:Path + ';{}', 'Machine')",
                install_dir.display()
            ),
        ])
        .status()?;
    if la == "J" {
        println!("インストール完了");
        println!("PATH追加");
    } else {
        println!("Sucess install");
        println!("Add PATH");
    }
    Ok(())
}
