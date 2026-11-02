use std::process::Command;

fn version_of(exe: &str) -> String {
    let out = Command::new(exe).arg("--version").output().unwrap();
    assert!(out.status.success(), "{exe} --version failed: {out:?}");
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn both_binaries_print_the_version() {
    let want = format!("mayu {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(version_of(env!("CARGO_BIN_EXE_mayu")), want);
    assert_eq!(version_of(env!("CARGO_BIN_EXE_mayu-run")), want);
}

#[test]
fn unimplemented_subcommands_exit_125() {
    let out = Command::new(env!("CARGO_BIN_EXE_mayu"))
        .arg("run")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(125));
}
