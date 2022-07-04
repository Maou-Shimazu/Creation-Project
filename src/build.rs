#![allow(unused_assignments)]
/// Build, handles the functions for the build process.
use crate::cppm;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    io::{self, Write},
    path::Path,
    process::*,
    str,
    time::Instant,
};

/// ### Struct used to serialze Cppm.toml
/// Usage:
/// ```rs
/// let value: LocalConfig = toml::from_str(&std::fs::read_to_string("Cppm.toml").unwrap
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub project: HashMap<String, String>,
}

/// Generates an include command based on all the folders passed into the argument.\
/// Used in [build](https://github.com/Maou-Shimazu/Cpp-Project-Manager/blob/main/src/build.rs#L56)
fn include(folders: Vec<&str>) -> String {
    let mut return_string: Vec<&str> = Vec::new();
    for i in folders {
        return_string.push(i);
    }
    format!("-I{}", return_string.join(" -I"))
}

/// Links libraries specified.\
/// Used in [build](https://github.com/Maou-Shimazu/Cpp-Project-Manager/blob/main/src/build.rs#L56)
fn library(libs: Vec<&str>) -> String {
    let mut return_string: Vec<&str> = Vec::new();
    for i in libs {
        return_string.push(i);
    }
    format!("-L{}", return_string.join(" -L"))
}

/// ### Struct used to serialze defaults file
/// Usage:
/// ```rs
/// let value: Def = toml::from_str(&std::fs::read_to_string(&cppm::defaults_file()).unwrap()).unwrap();
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    compilers: HashMap<String, String>,
}

// note: add `flags_all = bool`, `flags = ""`
// note: optimize for smart object building headerfiles in the future
pub fn build(release: bool, run_type: bool) {
    let start = Instant::now();
    if !Path::new("Cppm.toml").exists() {
        println!("Cppm project isnt in current directory!");
        exit(0);
    }
    if !Path::new(&defaults_file()).exists() {
        println!(
            "{}",
            "You haven't set up your defaults yet! Run `cppm --config` to resolve this error."
                .red()
        );
        exit(0);
    }
    let mut target = String::new();
    let mut build_t = String::new();

    let l: LocalConfig = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    #[cfg(windows)]
    let canc: String = std::fs::canonicalize(".")
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .replace('\\', "\\")
        .trim()[4..]
        .to_owned();
    #[cfg(unix)]
    let canc: String = std::fs::canonicalize(".")
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    println!(
        "   {} {} v{} ({})",
        "Compiling".bright_blue().bold(),
        l.project["name"],
        l.project["version"],
        canc
    );

    let cppm: LocalConfig = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    let compiler: Def = toml::from_str(&read_to_string(&cppm::defaults_file()).unwrap()).unwrap();

    let includes: Vec<&str> = cppm.project["include"].split(',').collect();
    let mut libraries: Vec<&str> = Vec::new(); // note: someone please test that the libraries link properly.
    if cppm.project.contains_key("libs") {
        libraries = cppm.project["libs"].split(',').collect();
    } else {
        libraries = vec![""];
    }
    let mut flags: Vec<&str> = vec![
        "-fdiagnostics-color=always",
        "-Wall",
        "-Wpedantic",
        "-Werror",
        "-Wshadow",
        "-Wformat=2",
        "-Wconversion",
        "-Wunused-parameter",
    ];
    if let Some(flag_string) = cppm.project.get("flags").filter(|f| !f.is_empty()) {
        flags = flag_string.split(" ").collect();
    }
    let src = cppm.project["src"].clone();
    let mut standard = cppm.project["standard"].clone();
    standard = format!("-std=c++{standard}");
    fs::create_dir_all("build").ok();

    let out: Output;

    if release {
        out = Command::new(&compiler.compilers["cpp"])
            .arg(standard.clone())
            .arg("-o")
            .arg(format!("build/{}", cppm.project["name"]))
            .arg(src.clone())
            .arg(include(includes.clone()))
            .arg(library(libraries.clone()))
            .arg("-O3")
            .args(flags)
            .output()
            .unwrap();
        target = "optimized".to_string();
        build_t = "release".to_string();
    } else {
        out = Command::new(&compiler.compilers["cpp"])
            .arg(standard.clone())
            .arg("-o")
            .arg(format!("build/{}", cppm.project["name"]))
            .arg(src.clone())
            .arg(include(includes.clone()))
            .arg(library(libraries.clone()))
            .args(flags)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        target = "unoptimized".to_owned();
        build_t = "dev".to_string();
    }
    // println!(
    //     "{} {standard} -o build/{} {src} {} {} {flags:?}",
    //     &compiler.compilers["cpp"],
    //     cppm.project["name"].clone(),
    //     include(includes.clone()),
    //     library(libraries.clone())
    // );

    if !out.status.success() {
        io::stdout().write_all(&out.stdout).unwrap();
        io::stderr().write_all(&out.stderr).unwrap();
        exit(0);
    }

    if !run_type {
        println!(
            "    {} {build_t} [{target}] target(s) in {:?}",
            "Finished".bright_blue().bold(),
            start.elapsed()
        );
    }
}

pub fn run(release: bool, run_type: bool, extra_args: Vec<String>) {
    if !Path::new("Cppm.toml").exists() {
        println!("Cppm project isnt in current directory!");
        exit(0);
    }
    if !Path::new(&defaults_file()).exists() {
        println!(
            "{}",
            "You haven't set up your defaults yet! Run `cppm --config` to resolve this error."
                .red()
        );
        exit(0);
    }
    let cppm: LocalConfig = toml::from_str(&read_to_string("Cppm.toml").unwrap()).unwrap();
    build(release, run_type);
    let l: LocalConfig = toml::from_str(&std::fs::read_to_string("Cppm.toml").unwrap()).unwrap();

    #[cfg(windows)]
    let name = format!("build/{}.exe", l.project["name"]);
    #[cfg(unix)]
    let name = format!("build/{}", l.project["name"]);
    #[cfg(windows)]
    let canc: String = std::fs::canonicalize(name)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .trim()[4..]
        .to_owned();
    #[cfg(unix)]
    let canc: String = std::fs::canonicalize(name)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    println!(
        "     {} `{} {}`",
        "Running".bright_blue().bold(),
        canc,
        extra_args.join(" ")
    );

    let run = format!("build/{}", cppm.project["name"]);
    let out = Command::new(run).args(extra_args).output().unwrap();
    io::stdout().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();
}

fn defaults_file() -> String {
    let defaultsdir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('"', "")
        .replace('\\', "/");
    format!("{}/.cppm/defaults.toml", defaultsdir)
}
