use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=web/dist/index.html");
    Command::new("yarn")
        .args(&["--cwd", "web"])
        .arg("build").status().unwrap();
}
