use assert_cmd::Command;
use predicates::prelude::*;

fn page() -> Command {
    Command::cargo_bin("page").unwrap()
}

fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn sanity() {
    page()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("page "));

    page()
        .arg("--help")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("page ").and(predicate::str::contains(
                "Password manager with age encryption",
            )),
        );
}

#[test]
fn info() {
    let dir = tempdir();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .assert()
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("info")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("Storage file: ").and(
                predicate::str::contains("entries.toml.age")
                    .and(predicate::str::contains("\n").count(1).trim()),
            ),
        );
}

#[test]
fn switch_storage_folder() {
    let dir = tempdir();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .assert()
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(format!(
            "Storage file: {}/entries.toml.age\n",
            dir.path().to_str().unwrap()
        )));
}

#[test]
fn new_show_list() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(predicate::str::starts_with("Passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .write_stdin(format!("{}\n{}\n{}", passphrase, entry, password))
        .assert()
        .stdout(format!("Passphrase: New entry: Password for {}: ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", password))
        .success();
}

#[test]
fn edit_entry() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(predicate::str::starts_with("Passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .write_stdin(format!("{}\n{}\n{}", passphrase, entry, password))
        .assert()
        .stdout(format!("Passphrase: New entry: Password for {}: ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", password))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .write_stdin(format!("{}\n{}\n", passphrase, new_password))
        .assert()
        .stdout("Enter passphrase: New password for editable: ")
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", new_password))
        .success();
}

#[test]
fn remove_entry() {
    let dir = tempdir();
    let passphrase = "donttell";
    let entry = "begone";
    let password = "pw";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(predicate::str::starts_with("Passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .write_stdin(format!("{}\n{}\n{}", passphrase, entry, password))
        .assert()
        .stdout(format!("Passphrase: New entry: Password for {}: ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg(entry)
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout("Enter passphrase: ")
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout("Enter passphrase: ")
        .success();
}

#[test]
fn fail_list_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_show_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("foo")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_new_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_edit_no_entry() {
    let dir = tempdir();
    let passphrase = "fail";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(predicate::str::starts_with("Passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg("404")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .failure()
        .stdout("Enter passphrase: ")
        .stderr("Error: entry not found: 404\n");
}

#[test]
fn fail_remove_no_entry() {
    let dir = tempdir();
    let passphrase = "no_entry_no_remove";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .stdout(predicate::str::starts_with("Passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg("no-entry")
        .write_stdin(format!("{}\n", passphrase))
        .assert()
        .failure()
        .stdout("Enter passphrase: ")
        .stderr("Error: entry not found: no-entry\n");
}
