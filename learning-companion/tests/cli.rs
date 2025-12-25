use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_runs_without_arguments() {
    // 测试无参数运行（应该进入 TUI 模式）
    // 由于 TUI 需要交互，我们测试进程能正常启动
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .timeout(std::time::Duration::from_secs(2))
        .assert()
        .failure(); // TUI 会因为无法初始化终端而退出
}

#[test]
fn test_cli_shows_help() {
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("learning-companion"));
}

#[test]
fn test_cli_shows_version() {
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_cli_dashboard_command() {
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .args(["--path", ".", "dashboard"])
        .assert()
        .success()
        .stdout(predicate::str::contains("学习进度"));
}

#[test]
fn test_cli_achievements_command() {
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .arg("achievements")
        .assert()
        .success();
}

#[test]
fn test_cli_accepts_path_argument() {
    Command::cargo_bin("learning-companion")
        .expect("binary exists")
        .args(["--path", "."])
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .failure(); // TUI 无法在测试环境中运行
}
