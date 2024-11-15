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
        .stdout(predicate::str::starts_with(
            "A password manager with age encryption",
        ));
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
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", password))
        .success();
}

#[test]
fn new_overwrite() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";
    let new_password = "new_password";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\ny\n{}", passphrase, new_password))
        .assert()
        .stdout(format!(
            "Enter passphrase: Entry '{}' already exists. Overwrite (y/N)?Password for '{}': ",
            entry, entry
        ))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", new_password))
        .success();
}

#[test]
fn new_show_attributes() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";
    let username = "username";
    let url = "url";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .arg("-u")
        .arg(username)
        .arg("--url")
        .arg(url)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg(entry)
        .arg("--on-screen")
        .arg("--attribute")
        .arg("username")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", username))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg(entry)
        .arg("--on-screen")
        .arg("--attribute")
        .arg("url")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", url))
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
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", password))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, new_password))
        .assert()
        .stdout(format!("Enter passphrase: New password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", new_password))
        .success();
}

#[test]
fn edit_entry_name() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let new_entry = "editable2";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", password))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .arg("-n")
        .arg(new_entry)
        .write_stdin(format!("{}\n{}", passphrase, new_password))
        .assert()
        .stdout(format!(
            "Enter passphrase: New password for '{}': ",
            new_entry
        ))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(new_entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", new_password))
        .success();
}

#[test]
fn edit_entry_overwrite() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let entry2 = "editable2";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry2)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry2))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .arg("-n")
        .arg(entry2)
        .write_stdin(format!("{}\nY\n{}", passphrase, new_password))
        .assert()
        .stdout(format!(
            "Enter passphrase: Entry '{}' already exists. Overwrite (y/N)?New password for '{}': ",
            entry2, entry2
        ))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", entry2))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry2)
        .write_stdin(format!("{}", passphrase))
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
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{}\n{}", passphrase, password))
        .assert()
        .stdout(format!("Enter passphrase: Password for '{}': ", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(format!("Enter passphrase: {}\n", entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg(entry)
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout("Enter passphrase: ")
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(format!("{}", passphrase))
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
        .arg("entry")
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
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg("404")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .failure()
        .stdout("Enter passphrase: ")
        .stderr("Error: entry '404' not found\n");
}

#[test]
fn fail_remove_no_entry() {
    let dir = tempdir();
    let passphrase = "no_entry_no_remove";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg("no-entry")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .failure()
        .stdout("Enter passphrase: ")
        .stderr("Error: entry 'no-entry' not found\n");
}
