//! `lc` — tiny CLI for the LeetCode playground.
//!
//!   lc new <id> <title...>   create src/problems/pNNNN_slug.rs (uses clipboard starter code if present)
//!   lc test [id]             run tests for a problem (default: most recently edited)
//!   lc watch [id]            re-run tests on every file save (needs cargo-watch)
//!   lc list                  list all problems
//!   lc rm <id>               delete a problem

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

fn problems_dir() -> PathBuf {
    let root = Path::new(ROOT);
    root.canonicalize().unwrap_or(root.to_path_buf()).join("src/problems")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let rest = &args[args.len().min(1)..];
    match cmd {
        "new" | "n" => new(rest),
        "test" | "t" => test(rest, false),
        "watch" | "w" => test(rest, true),
        "list" | "ls" => list(),
        "rm" => rm(rest),
        _ => help(),
    }
}

fn help() {
    println!(
        "lc — LeetCode playground

USAGE:
  lc new <id> <title...> [--no-paste]   Create a problem file. If your clipboard holds LeetCode's
                                        Rust starter code (contains `impl Solution`), it is used.
  lc test  [id]                         Run tests for a problem (default: last edited).
  lc watch [id]                         Re-run tests on every save (needs `cargo install cargo-watch`).
  lc list                               List problems.
  lc rm <id>                            Delete a problem.

EXAMPLE:
  (copy the Rust starter code on leetcode.com)
  lc new 1 Two Sum
  lc test"
    );
}

// ---------------------------------------------------------------------------

fn new(args: &[String]) {
    let no_paste = args.iter().any(|a| a == "--no-paste");
    let args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if args.len() < 2 {
        die("usage: lc new <id> <title...>   e.g. lc new 1 Two Sum");
    }
    let id: u32 = args[0].parse().unwrap_or_else(|_| die("id must be a number"));
    if find_problem(id).is_some() {
        die(&format!("problem {id} already exists: {}", find_problem(id).unwrap().display()));
    }
    let title = args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
    let slug = slugify(&title);
    let module = format!("p{id:04}_{slug}");
    let path = problems_dir().join(format!("{module}.rs"));

    let starter = if no_paste { None } else { clipboard().filter(|c| c.contains("impl Solution")) };
    let from_clipboard = starter.is_some();
    let starter = starter.map(|s| fill_empty_bodies(&s)).unwrap_or_else(|| {
        "impl Solution {\n    pub fn solve(nums: Vec<i32>) -> i32 {\n        todo!()\n    }\n}\n".to_string()
    });
    let fn_name = first_fn_name(&starter).unwrap_or_else(|| "solve".into());
    let params = param_names(&starter, &fn_name).join(", ");
    let starter = strip_leading_comment_block(&starter);

    let url_slug = slug.replace('_', "-");
    let content = format!(
        r#"//! {id}. {title}
//! https://leetcode.com/problems/{url_slug}/

#![allow(unused_imports, unused_variables, unused_mut, dead_code, unreachable_code)]
use crate::util::*;
use crate::{{cases, list, s, tree, vs, vvc}};
use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

pub struct Solution;

{starter}
#[cfg(test)]
mod tests {{
    #[allow(unused_imports)]
    use super::*;

    // One line per test case:  name: (args...) => expected,
    // Helpers: list![1,2], tree![1,null,2], s!("abc"), vs!["a","b"]
    cases! {{
        Solution::{fn_name};
        // ex1: ({params}) => expected,
    }}
}}
"#,
        starter = starter.trim_end().to_string() + "\n",
    );

    fs::write(&path, content).unwrap_or_else(|e| die(&format!("write failed: {e}")));
    register_module(&module);

    println!("created {}", path.display());
    if from_clipboard {
        println!("  starter code taken from clipboard (fn `{fn_name}`)");
    } else {
        println!("  no LeetCode starter code found in clipboard — using a placeholder signature");
    }
    println!("  next: edit the file, fill in test cases, then run `lc test`");
}

fn test(args: &[String], watch: bool) {
    let path = match args.first() {
        Some(id) => {
            let id: u32 = id.parse().unwrap_or_else(|_| die("id must be a number"));
            find_problem(id).unwrap_or_else(|| die(&format!("problem {id} not found")))
        }
        None => latest_problem().unwrap_or_else(|| die("no problems yet — run `lc new`")),
    };
    let module = path.file_stem().unwrap().to_string_lossy().to_string();
    let test_cmd = format!("test -q --lib problems::{module}:: -- --nocapture");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(ROOT);
    if watch {
        println!("watching {module} (re-runs tests on every save, Ctrl+C to stop)");
        cmd.args(["watch", "-q", "-c", "-x", &test_cmd]);
    } else {
        println!("testing {module}");
        cmd.args(test_cmd.split(' '));
    }
    let status = cmd
        .status()
        .unwrap_or_else(|e| die(&format!("failed to run cargo: {e}")));
    exit(status.code().unwrap_or(1));
}

fn list() {
    let mut files = problem_files();
    files.sort();
    if files.is_empty() {
        println!("no problems yet — run `lc new <id> <title>`");
    }
    for f in files {
        println!("{}", f.file_stem().unwrap().to_string_lossy());
    }
}

fn rm(args: &[String]) {
    let id: u32 = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| die("usage: lc rm <id>"));
    let path = find_problem(id).unwrap_or_else(|| die(&format!("problem {id} not found")));
    let module = path.file_stem().unwrap().to_string_lossy().to_string();
    fs::remove_file(&path).unwrap();
    let mod_rs = problems_dir().join("mod.rs");
    let text = fs::read_to_string(&mod_rs).unwrap();
    let line = format!("pub mod {module};");
    let text: Vec<&str> = text.lines().filter(|l| l.trim() != line).collect();
    fs::write(&mod_rs, text.join("\n") + "\n").unwrap();
    println!("removed {module}");
}

// ---------------------------------------------------------------------------

fn problem_files() -> Vec<PathBuf> {
    fs::read_dir(problems_dir())
        .map(|rd| {
            rd.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| {
                    p.extension().is_some_and(|e| e == "rs")
                        && p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('p'))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn find_problem(id: u32) -> Option<PathBuf> {
    let prefix = format!("p{id:04}_");
    problem_files()
        .into_iter()
        .find(|p| p.file_name().unwrap().to_string_lossy().starts_with(&prefix))
}

fn latest_problem() -> Option<PathBuf> {
    problem_files()
        .into_iter()
        .max_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok())
}

fn register_module(module: &str) {
    let mod_rs = problems_dir().join("mod.rs");
    let text = fs::read_to_string(&mod_rs).unwrap_or_default();
    let mut header: Vec<String> = vec![];
    let mut mods: Vec<String> = vec![];
    for l in text.lines() {
        if l.trim_start().starts_with("pub mod ") {
            mods.push(l.trim().to_string());
        } else if !l.trim().is_empty() {
            header.push(l.to_string());
        }
    }
    mods.push(format!("pub mod {module};"));
    mods.sort();
    mods.dedup();
    let mut out = header.join("\n");
    if !out.is_empty() {
        out.push_str("\n\n");
    }
    out.push_str(&mods.join("\n"));
    out.push('\n');
    fs::write(mod_rs, out).unwrap();
}

fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn clipboard() -> Option<String> {
    let out = Command::new("pbpaste").output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).to_string())
}

fn first_fn_name(code: &str) -> Option<String> {
    let i = code.find("pub fn ")? + "pub fn ".len();
    let name: String = code[i..].chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    (!name.is_empty()).then_some(name)
}

/// Parameter names of `fn_name`, e.g. `["nums", "target"]`.
fn param_names(code: &str, fn_name: &str) -> Vec<String> {
    let Some(i) = code.find(&format!("fn {fn_name}(")) else { return vec![] };
    let mut depth = 0;
    let mut params = vec![String::new()];
    for c in code[i..].chars().skip_while(|&c| c != '(').skip(1) {
        match c {
            ')' if depth == 0 => break,
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' => depth -= 1,
            ',' if depth == 0 => params.push(String::new()),
            _ => {}
        }
        if depth == 0 && c != ',' {
            params.last_mut().unwrap().push(c);
        }
    }
    params
        .iter()
        .filter_map(|p| p.split(':').next())
        .map(|n| n.trim().trim_start_matches("mut ").to_string())
        .filter(|n| !n.is_empty())
        .collect()
}

/// LeetCode's starter has empty bodies `{\n        \n    }` which don't compile; put `todo!()` in them.
fn fill_empty_bodies(code: &str) -> String {
    let mut out = String::new();
    let mut rest = code;
    while let Some(i) = rest.find("pub fn ") {
        let Some(open) = rest[i..].find('{').map(|o| i + o) else { break };
        out.push_str(&rest[..=open]);
        rest = &rest[open + 1..];
        let ws = rest.len() - rest.trim_start().len();
        if rest[ws..].starts_with('}') {
            out.push_str("\n        todo!()\n    ");
            rest = &rest[ws..];
        }
    }
    out.push_str(rest);
    out
}

/// Drop LeetCode's commented-out `ListNode`/`TreeNode` definitions (we provide them in util).
fn strip_leading_comment_block(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let start = lines
        .iter()
        .position(|l| {
            let t = l.trim();
            !(t.is_empty() || t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .unwrap_or(0);
    // `use` lines are dropped too: the template already imports Rc, RefCell, collections, util.
    lines[start..]
        .iter()
        .filter(|l| !l.trim_start().starts_with("use "))
        .copied()
        .collect::<Vec<_>>()
        .join("\n")
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    exit(1)
}
