use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn bin() -> Command {
    Command::cargo_bin("hr-cli").expect("binary exists")
}

#[test]
fn e2e_add_list_search_remove() {
    // Use a temp directory for storage and point HR_STORAGE_PATH to it
    let tmp = tempdir().expect("tempdir");
    let storage_root = tmp.path().to_string_lossy().to_string();

    // add alice
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .args([
            "add",
            "--name",
            "alice",
            "--label",
            "eng",
            "--label",
            "oncall",
            "--metric",
            "speed:10",
            "--metric",
            "height:20",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Adding alice"));

    // add alina
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .args([
            "add",
            "--name",
            "alina",
            "--label",
            "eng",
            "--metric",
            "speed:11",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Adding alina"));

    // list should include both
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found human: alice").and(predicate::str::contains("Found human: alina")),
        );

    // search by wildcard + label + metric threshold
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .args([
            "search",
            "--name",
            "ali*",
            "--label",
            "eng",
            "--metric",
            "speed:10",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found human: alice").and(predicate::str::contains("Found human: alina")),
        );

    // remove alice
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .args(["remove", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removing alice"));

    // list should only include alina now
    bin()
        .env("HR_STORAGE_PATH", &storage_root)
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found human: alina").and(predicate::str::contains("Found human: alice").not()),
        );
}
