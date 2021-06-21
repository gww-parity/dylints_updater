use ansi_term::Colour;
//use anyhow::Result;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::env::consts::{DLL_PREFIX,DLL_SUFFIX};

#[derive(derive_new::new)]
pub struct DylintsUpdaterConfig {
    dylint_library_path: String,
}

/// executes closure on each directory
pub fn visit_subdirectories(
    dir: &Path,
    recursively: bool,
    cb: &dyn Fn(&DirEntry),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                cb(&entry);
                if recursively {
                    visit_subdirectories(&path, recursively, cb)?;
                }
            }
        }
    }
    Ok(())
}

/// Does filename match "DLL_PREFIX library_name @ toolchain DLL_SUFFIX" pattern expected by Dylint.
pub fn is_filename_like_dylint_expects(path: &Path) -> bool {
    // TODO: make it more robouts, to also capture if toolchain matches regexp
    let f = match path.file_name() {
        Some(fn_os_str) => match fn_os_str.to_str() {
                Some(s) => s.to_string(),
                None => return false,
        },
        None => return false,
    };
    f.starts_with(DLL_PREFIX) && f.ends_with(DLL_SUFFIX) && f.contains("@")
}

/// Is there dylint lint project directory under given path? It's just far from perfect heuristic.
pub fn is_dylint_project_directory(path: &Path) -> bool {
    if path.is_dir() == false {
        return false;
    }
    let expected_files = ["Cargo.toml", "src/lib.rs"];
    for f in &expected_files {
        let filepath = path.join(f);
        if filepath.is_file() == false {
            return false;
        }
    }
    return true;
}

pub enum CmdRet {
    Output(process::Output),
    Ok(()),
    Err(anyhow::Error),
}

pub fn cmd_and_1arg(lint_project_path: &Path, cmd: &str, arg: &str) -> CmdRet {
    // TODO: add flags/config to allow configuration if user wants to use developer or release build
    // TODO: make --no-capture flag that would output/stream stdout and stderr from those subprocesses
    CmdRet::Output(
        process::Command::new(cmd)
            .current_dir(lint_project_path)
            .stdin(process::Stdio::null())
            .arg(arg)
            .output()
            .expect(&format!("failed to run `{} {}`", cmd, arg)),
    )
}

pub fn cargo_clean(lint_project_path: &Path) -> CmdRet {
    cmd_and_1arg(lint_project_path, "cargo", "clean")
}

pub fn cargo_build(lint_project_path: &Path) -> CmdRet {
    cmd_and_1arg(lint_project_path, "cargo", "build")
}

pub fn update_in_lints_cache(lint_project_path: &Path, lints_cache_path: &Path) -> CmdRet {
    print!(
        "<<update_in_lints_cache( {:?}, {:?} )>>",
        lint_project_path, lints_cache_path
    );
    if lints_cache_path.is_dir() == false {
        return CmdRet::Err(anyhow::anyhow!(
            "DYLINT_LIBRARY_PATH given as `{:?}` is not a directory",
            lints_cache_path
        ));
    }
    // running from closure to convert Result into CmdRet TODO: consider implementing From/Into trait
    let cp_files = |dir_path:&Path| -> anyhow::Result<()> {
        let mut first_match = true;
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let fpath = entry.path();
            if fpath.is_file() {
                if is_filename_like_dylint_expects(&fpath) {
                    if first_match { println!(""); first_match = false; }
                    // TODO control verbosity (even soncider on maximum verbosity to print not matched items)
                    if let Some(filename) = fpath.file_name() {
                        println!("copying `{:?}` ", filename);
                        // TODO function will overwrite destination, consider flag that makes it interactive or skips (no overwrite)
                        let dst = lints_cache_path.join(filename);
                        fs::copy(fpath, dst)?;
                    }
                }
            }
        };
        Ok(())
    };
    // TODO: consider supporting release builds of lints. For now debug build lints should be good.
    let project_target_debug_path = lint_project_path.join("target").join("debug");
    match cp_files(&project_target_debug_path) {
        Ok(()) => { return CmdRet::Ok(()) },
        Err(e) => { println!("ERROR: {:?}", e); return CmdRet::Err(e) },
    }
}

#[derive(derive_new::new)]
pub struct OpRet {
    operation_str: String,
    output: Option<process::Output>,
}

pub fn clean_rebuild_and_update_cache(
    dir: &DirEntry,
    verbose: bool,
    lints_cache_path: &Path,
) -> Result<OpRet, OpRet> {
    // TODO: more verbosity levels, that allow priting stdout, stderr
    let project_path = dir.path();
    if is_dylint_project_directory(&project_path) == false {
        return Err(OpRet::new("is_it_dylint_project_directory".into(), None));
    }
    if lints_cache_path.is_dir() == false {
        return Err(OpRet::new("is_lints_cache_path_a_directory".into(), None));
    }
    let update_in_lints_cache_curriedfx = |p: &Path| update_in_lints_cache(p, lints_cache_path);
    #[rustfmt::skip]
    let commands_chain: Vec<(&str, &dyn Fn(&Path) -> CmdRet)> = vec![
        ("cargo clean", &cargo_clean),
        ("cargo build", &cargo_build),
        ("update libraries in dylint cache", &update_in_lints_cache_curriedfx),
        ];
    for (cmd_str, cmd_fx) in commands_chain {
        if verbose {
            print!("{}...", cmd_str);
            let _ignore_flush_ret = io::stdout().flush();
        }
        match cmd_fx(&project_path) {
            CmdRet::Output(output) => {
                if output.status.success() == false {
                    return Err(OpRet::new(cmd_str.into(), Some(output)));
                }
            },
            CmdRet::Ok(_) => {},
            CmdRet::Err(_) => {
                return Err(OpRet::new(cmd_str.into(), None));
            },
        }
    }
    Ok(OpRet::new("clean_rebuild_and_update_cache".into(), None))
}

pub fn run(config: &DylintsUpdaterConfig) -> anyhow::Result<()> {
    // TODO check if there is parameter "rebuild_and_update" specified
    let wd = Path::new(".");
    let lints_cache = Path::new(&config.dylint_library_path);
    let execute_and_ignore_output = |d: &DirEntry| {
        let p = d.path();
        print!("{:?}...", p);
        let _ignore_flush_ret = io::stdout().flush();
        if is_dylint_project_directory(&p) == false {
            println!(
                "does not look like project directory; {}",
                Colour::White.dimmed().paint("Skipping directory.")
            );
            return;
        }
        let error_colored_str = Colour::Red.bold().paint("Error");
        match clean_rebuild_and_update_cache(d, true, lints_cache) {
            Ok(_op_ret) => {
                println!("{}", Colour::Green.paint("Done"));
            }
            Err(op_ret) => {
                match op_ret.output {
                    Some(output) => {
                        // TODO: make parameter that would allow more verbose reporting, e.g. with stdout and stderr.
                        match output.status.code() {
                            Some(exit_code) => println!(
                                "{} failed with status code {}.",
                                error_colored_str, exit_code
                            ),
                            None => println!("{} terminated by signal.", error_colored_str),
                        }
                    }
                    None => {
                        println!(
                            "{} during operation `{}`",
                            error_colored_str, op_ret.operation_str
                        );
                        return;
                    }
                }
            }
        };
        println!("{} for path: {:?}", Colour::Green.paint("Done"), p);
    }; // end of closure
    visit_subdirectories(wd, false, &execute_and_ignore_output)?;
    Ok(())
}
