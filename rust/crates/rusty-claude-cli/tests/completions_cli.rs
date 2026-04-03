use std::process::{Command, Output};

#[test]
fn bash_completions_emit_claw_entries() {
    let output = Command::new(env!("CARGO_BIN_EXE_claw"))
        .args(["completions", "bash"])
        .output()
        .expect("claw should launch");

    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("claw"));
    assert!(stdout.contains("complete"));
}

#[test]
fn completions_reject_unknown_shells() {
    let output = Command::new(env!("CARGO_BIN_EXE_claw"))
        .args(["completions", "fish"])
        .output()
        .expect("claw should launch");

    assert!(
        !output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("supported shells"));
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
