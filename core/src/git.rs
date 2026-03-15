use std::process::Command;

pub fn status() -> String {

    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to run git");

    String::from_utf8_lossy(&output.stdout).to_string()
}