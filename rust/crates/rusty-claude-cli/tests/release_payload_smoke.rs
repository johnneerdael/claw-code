use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[test]
fn staging_script_writes_binary_and_completion_assets() {
    let temp_dir = unique_temp_dir("release-payload");
    let output_dir = temp_dir.join("payload");
    fs::create_dir_all(&temp_dir).expect("temp dir should exist");

    let output = Command::new("python3")
        .current_dir(workspace_root())
        .args([
            "scripts/stage_release_payload.py",
            "--binary",
            env!("CARGO_BIN_EXE_claw"),
            "--target",
            "x86_64-apple-darwin",
            "--output",
            output_dir.to_str().expect("utf8 output path"),
        ])
        .output()
        .expect("staging script should launch");

    assert_success(&output);
    assert!(output_dir.join("bin").join("claw").is_file());
    assert!(output_dir
        .join("completions")
        .join("bash")
        .join("claw")
        .is_file());
    assert!(output_dir
        .join("completions")
        .join("zsh")
        .join("_claw")
        .is_file());
    assert!(output_dir
        .join("completions")
        .join("powershell")
        .join("claw.ps1")
        .is_file());
    assert!(output_dir.join("docs").join("README.md").is_file());

    fs::remove_dir_all(temp_dir).expect("cleanup temp dir");
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should resolve")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_millis();
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "claw-{label}-{}-{millis}-{counter}",
        std::process::id()
    ))
}
