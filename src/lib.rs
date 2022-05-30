#![feature(iter_intersperse)]
use nix::unistd::geteuid;
use std::fs;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use walkdir::{DirEntry, WalkDir};

pub fn check() {
    match Command::new("which")
        .arg("lynis")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .unwrap()
        .success()
    {
        true => (),
        false => {
            println!("Lynis not found in $PATH");
            println!("Please install lynis");
            exit(1);
        }
    }

    if geteuid().as_raw() != 0 {
        println!("Run this program as root!");
        exit(1);
    }

    let functions_file = fs::read_to_string("/usr/share/lynis/include/functions")
        .expect("lynis is installed => this file should exist");
    if functions_file.contains("b80012851cf037c6d8adda328907d400c95773958fb4fec4e544a02cd5eeab0e") {
        exit(1);
    }
}

pub fn find_files_in_folder(folder: &str) -> Vec<DirEntry> {
    WalkDir::new(folder)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter_map(|file| match file.metadata().unwrap().is_file() {
            true => Some(file),
            false => None,
        })
        .collect()
}

pub fn edit_file(file: &DirEntry) {
    let file_path = file.path().to_string_lossy();
    let file_path = file_path.as_ref();
    let file_contents = fs::read_to_string(file_path).unwrap();

    let mut new_contents = String::new();
    for (nr, line) in file_contents.lines().enumerate() {
        let editied_line = edit_line(file_path, line, nr + 1);
        new_contents.push_str(editied_line.as_str());
        new_contents.push('\n');
    }

    fs::write(file_path, new_contents).unwrap();
}

fn edit_line(filename: &str, line: &str, nr: usize) -> String {
    line.split(|c: char| c == ' ')
        .map(|token| match token {
            "AddHP" => token.to_string() + &format!(" \"{filename}:{nr}\""),
            _ => token.to_string(),
        })
        .intersperse(" ".to_string())
        .collect()
}

#[allow(non_snake_case)]
pub fn modify_lynis_AddHP() {
    let str = r#"
AddHP() {
#b80012851cf037c6d8adda328907d400c95773958fb4fec4e544a02cd5eeab0e
    HPADD=$2; HPADDMAX=$3
    HPPOINTS=$((HPPOINTS + HPADD))
    HPTOTAL=$((HPTOTAL + HPADDMAX))
    tput setaf 0
    tput bold
    if [ ${HPADD} -eq ${HPADDMAX} ]; then
        tput setab 2
        echo -n assigning points 'file:line' $1 " "
        echo "Hardening: assigned maximum for this item (${HPADDMAX}) ${HPPOINTS} / ${HPTOTAL} $(tput sgr0)"
    else
        tput setab 1
        echo -n assigning points 'file:line' $1 " "
        echo "Hardening: assigned partial (${HPADD} of ${HPADDMAX}) ${HPPOINTS} / ${HPTOTAL} $(tput sgr0)"
    fi
}
"#;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/usr/share/lynis/include/functions")
        .unwrap();
    file.write_all(str.as_bytes()).unwrap();
}
