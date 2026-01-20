use rayon::prelude::*;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use rayon::iter::IntoParallelRefIterator;

struct Cli {
    root_path: Option<String>,
}

fn resolve_args() -> Result<Cli, io::Error> {
    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).map(|f| f.clone());
    Ok(Cli { root_path: path })
}

fn find_cargo_project(path: PathBuf) -> Option<Vec<PathBuf>> {
    let (dir_list, file_list): (Vec<_>, Vec<_>) = fs::read_dir(&path)
        .ok()?
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .partition(|f| f.is_dir());

    // 引数パスのフォルダがCargoプロジェクトかどうか?
    if file_list
        .iter()
        .any(|f| f.file_name().unwrap().to_str().unwrap() == "Cargo.toml")
    {
        return Some(vec![path]);
    }

    // 内部のフォルダを再帰的に探索
    let res = dir_list
        .iter()
        .map(|f| find_cargo_project(f.to_path_buf()))
        .filter_map(|f| f)
        .flatten()
        .collect::<Vec<_>>();
    Some(res)
}

fn clean(path: PathBuf) -> anyhow::Result<String> {
    let res = {
        // std::env::set_current_dir(path)?;
        std::process::Command::new("cargo")
            .current_dir(path)
            .args(["clean"])
            .output()?
    };
    let r = { vec![res.stdout, res.stderr] }
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(String::from_utf8_lossy(&r).to_string())
}

fn main() {
    let cli = resolve_args().unwrap();
    let find_root_path = match cli.root_path {
        Some(v) => PathBuf::from(v),

        None => env::current_dir().unwrap(),
    };

    let list = find_cargo_project(find_root_path).unwrap();
    let show_list = list
        .iter()
        .map(|f| f.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("found these fodlers\n{show_list}\nok?(y/n)");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    if buf.trim() != "y" {
        return;
    }

    let result = list
        .par_iter()
        .map(|f| clean(f.to_path_buf()))
        .filter_map(|f| f.ok())
        .collect::<String>();

    println!("cleaned\n{result}");
}
