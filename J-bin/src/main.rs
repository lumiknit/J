/* lumiknit's jump helper
 * Authour: lumiknit (aasr4r4@gmail.com)
 * Version: 0.0.2 (230307) */

use std::env;
use std::process::{exit, Command};
use std::path::Path;

struct Env {
    repos_path: String,
    find_paths: Vec<String>,
    ignores: Vec<String>,
}

impl Env {
    fn new() -> Env {
        let home = env::var("J_HOME").unwrap();
        let repos_path = env::var("J_REPOS").unwrap_or(format!("{}/repos", home));
        let find_paths = env::var("J_FIND_PATHS")
            .unwrap_or(format!("{}/repos:{}/workspace", home, home))
            .split(":")
            .map(|s| s.to_string())
            .collect();
        let default_ignores =
            "__pycache__:bin:build:config:configs:coverage:dist:doc:docs:include:includes:li:libs:log:module:modules:node_modules:out:src:srcs:target:test:tests:tmp:vendor:vscode";
        let ignores = env::var("J_IGNORES")
            .unwrap_or(default_ignores.to_string())
            .split(":")
            .map(|s| s.to_string())
            .collect();
        Env {
            repos_path,
            find_paths,
            ignores,
        }
    }
}

fn clone(repo_url: String) {
    let env = Env::new();
    // Split repo_url by protocal
    let url_clone = repo_url.clone();
    let splitted: Vec<&str> = url_clone.split("://").collect();
    if splitted.len() != 2 {
        println!("Invalid repo url: {}", repo_url);
        exit(1)
    }
    // Mkdir
    Command::new("mkdir")
        .arg("-p")
        .arg(format!("{}/{}", env.repos_path, splitted[1]))
        .spawn()
        .expect("Failed to create directory")
        .wait()
        .unwrap();
    // Clone repo
    Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(format!("{}/{}", env.repos_path, splitted[1]))
        .spawn()
        .expect("Failed to clone repo")
        .wait()
        .unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    path: String,
    name: String,
    distance: u64,
}

fn gather_directories(
    env: &Env,
    dirs: &mut Vec<Dir>,
    root: &String,
    dir: &Path,
) {
    if !dir.is_dir() {
        return;
    }
    let filename = dir.file_name().unwrap().to_str().unwrap();
    if env.ignores.contains(&filename.to_string()) {
        return;
    }
    // Push dir to dirs
    let dir_string = dir.to_str().unwrap().to_string();
    let name = dir_string.replace(root, "");
    dirs.push(Dir {
        path: dir_string.clone(),
        name,
        distance: 0,
    });
    // Check if dir contains .git directory
    let git_dir = format!("{}/.git", dir_string);
    if Path::new(&git_dir).is_dir() {
        return;
    }
    // Gather sub-directories
    if let Ok(sub_dirs) = dir.read_dir() {
        for entry in sub_dirs {
            if let Ok(entry) = entry {
                let path = entry.path();
                gather_directories(env, dirs, root, &path);
            }
        }
    }
}

fn min_edit_distance(a: &str, b: &str) -> u64 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let insertion_cost: u64 = 10;
    let deletion_cost: u64 = 4000;
    let substitution_cost: u64 = 3000;
    let case_cost = 1;
    let transpose_cost = 3;

    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        dp[i][0] = (i as u64) * deletion_cost;
    }
    for j in 0..=b.len() {
        dp[0][j] = (j as u64) * insertion_cost;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            // Costs of insertion, deletion, substitution
            let mut costs = vec![
                dp[i - 1][j] + deletion_cost,
                dp[i][j - 1] + insertion_cost,
                dp[i - 1][j - 1] + substitution_cost,
            ];
            // Cost if exactly same character
            if a_bytes[i - 1] == b_bytes[j - 1] {
                costs.push(dp[i - 1][j - 1]);
            }
            // Cost if case is different
            if a_bytes[i - 1].to_ascii_lowercase() == b_bytes[j - 1].to_ascii_lowercase() {
                costs.push(dp[i - 1][j - 1] + case_cost);
            }
            // Cost if transposed
            if i > 1 &&
                j > 1 &&
                a_bytes[i - 1].to_ascii_lowercase() ==
                    b_bytes[j - 2].to_ascii_lowercase() &&
                a_bytes[i - 2].to_ascii_lowercase() ==
                    b_bytes[j - 1].to_ascii_lowercase() {
                costs.push(dp[i - 2][j - 2] + transpose_cost);
            }
            dp[i][j] = *costs.iter().min().unwrap_or(&u64::MAX);
        }
    }
    dp[a.len()][b.len()]
}

fn find(keyword: String) {
    let env = Env::new();
    let mut dirs = vec![];
    // Find all subpaths
    let find_paths = env.find_paths.clone();
    for path in find_paths {
        let dir = Path::new(&path);
        gather_directories(&env, &mut dirs, &path, dir);
    }
    // Calculate distances
    for dir in dirs.iter_mut() {
        dir.distance = min_edit_distance(&keyword, &dir.name);
    }
    dirs.sort_by(|a, b| a.distance.cmp(&b.distance));
    // Print results
    for dir in dirs {
        println!("{}", dir.path);
    }
}

fn print_help(args: Vec<String>) {
    println!("J Binary");
    println!("Usage: {} <command> <args>", args[0]);
    println!("Commands:");
    println!("  help           - Print this help message");
    println!("  clone <url>    - Clone a remote repository to local");
    println!("  find <keyword> - Fuzzy-find a directory");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_help(args);
        return;
    }
    match args[1].as_str() {
        "clone" => clone(args[2].clone()),
        "find" => find(args[2].clone()),
        _ => print_help(args),
    }
}
