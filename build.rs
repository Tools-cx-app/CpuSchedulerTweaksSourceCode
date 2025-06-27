use std::{env, fs::File, io::Write, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Failed to get $OUT_DIR");
    let out_dir = Path::new(&out_dir);

    let output = Command::new("git")
        .args(["rev-list", "--count", "--all"])
        .output()
        .unwrap();

    let version = String::from_utf8_lossy(&output.stdout);

    File::create(Path::new(out_dir).join("VERSION"))
        .unwrap()
        .write_all(version.as_bytes())
        .unwrap();
}
