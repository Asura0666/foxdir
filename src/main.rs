use chrono::{DateTime, Local};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Bytes"}]
    len_bytes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = "A fast and colorful Rust CLI tool to explore directories — view file details as elegant tables or clean JSON output.",
    long_about = "DirView is a lightweight, blazing-fast command-line tool built in Rust for exploring directories with style.
It displays file names, types, sizes, and modification times in a beautifully formatted colorized table, or exports the same data as JSON for programmatic use.
Ideal for developers who want a clean, fast, and informative CLI experience without unnecessary bloat."
)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let path = cli
        .path
        .unwrap_or_else(|| PathBuf::from(r"/mnt/d/Rust/Projects/foxdir"));
    println!("selected path: {}", path.display().green());

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let get_files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&get_files).unwrap_or("cannot parse json".to_string())
                )
            } else {
                print_table(path);
            }
        } else {
            eprintln!("{}", "The Path does not exist...!".red());
        }
    } else {
        eprintln!("{}", "can't read directory".red());
    }
}

fn print_table(path: PathBuf) {
    let files = get_files(&path);
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(1), Color::FG_BRIGHT_WHITE);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);

    println!("{}", table);
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for file in read_dir.flatten() {
            map_data(file, &mut data);
        }
    }

    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(file.path()) {
        let modified_time = meta
            .modified()
            .ok()
            .map(DateTime::<Local>::from)
            .map_or_else(
                || "unknown".to_string(),
                |dt| dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            );

        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: modified_time,
        });
    }
}
