use clap::Parser;
use std::error::Error;
use std::io::{Lines, Write};
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

// clap crateを使って、コマンドライン引数に型をつける
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // 書き込みも効率化させる
    // get the global stdout entity
    let stdout = io::stdout();
    // let mut handle = stdout.lock(); // これでもシステムが毎回stdoutをlockしたり、unlockしたりが防げる
    let mut handle = io::BufWriter::new(stdout);

    let lines = read_lines(&args.path);
    let lines = match lines {
        Ok(lines) => lines,
        Err(e) => return Err(e.into()),
    };

    find_matches(lines, &args.pattern, &mut handle)?;
    Ok(())
}

fn read_lines<P: AsRef<Path>>(filepath: P) -> Result<io::Lines<BufReader<File>>, Box<dyn Error>> {
    // ファイルの中身を読んでみる
    let f = File::open(filepath).expect("File not found");
    let f = BufReader::new(f).lines();
    Ok(f)
}

// さまざまな種類の出力先を受け取ること
fn find_matches<W: Write>(
    lines: Lines<BufReader<File>>,
    pattern: &str,
    handle: &mut W,
) -> Result<(), Box<dyn Error>> {
    for line in lines {
        if let Ok(line) = line {
            if line.contains(pattern) {
                writeln!(handle, "{}", line)?;
            }
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let file = File::open("./README.md").unwrap(); // README.mdをテストに読み取る
    let lines = BufReader::new(file).lines();
    let mut result = Vec::new();
    find_matches(lines, "build", &mut result).unwrap();
    //  b prefix makes this a byte string literal so its type is going to be &[u8]
    assert_eq!(result, b"cargo build\n"); // stdout expects bytes (not strings)
}

#[test]
fn file_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("day1")?;
    cmd.arg("everyone").arg("/file/does/not/exists");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
    Ok(())
}
