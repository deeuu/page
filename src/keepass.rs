use crate::entries::{Entry, Storage};
use anyhow::Result;
use keepass::{db::Group, db::Node, Database, DatabaseKey};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

pub fn new_credentials(keyfile: Option<&PathBuf>, password: bool) -> Result<DatabaseKey> {
    let mut key = DatabaseKey::new();

    if password {
        let pw = rpassword::prompt_password_stdout("Enter keepass password: ")?;
        key = key.with_password(&pw);
    }

    if let Some(path) = keyfile {
        let keyfile = &mut File::open(path)?;
        key = key.with_keyfile(keyfile)?;
    }

    Ok(key)
}

impl Storage {
    pub fn from_keepass(path: &PathBuf, key: DatabaseKey, prefix: Option<&str>) -> Result<Storage> {
        let db = Database::open(&mut File::open(path)?, key)?;
        let mut entries = HashMap::new();
        keepass_entries(&mut entries, &db.root, prefix);
        Ok(Storage { entries })
    }
}

fn keepass_entries(entries: &mut HashMap<String, Entry>, group: &Group, group_path: Option<&str>) {
    for node in &group.children {
        match node {
            Node::Group(g) => {
                let new_path = match group_path {
                    Some(path) => format!("{}/{}", path, g.name),
                    None => g.name.to_string(),
                };
                keepass_entries(entries, g, Some(&new_path));
            }
            Node::Entry(node) => {
                let title = match node.get_title() {
                    Some(t) => t.to_string(),
                    None => node.get_uuid().to_string(),
                };

                let entry_name = match group_path {
                    Some(path) => format!("{}/{}", path, title),
                    None => title.to_string(),
                };

                if let Some(password) = node.get_password() {
                    let entry = Entry {
                        password: password.to_string(),
                        username: node.get_username().map(|s| s.to_string()),
                        url: node.get_url().map(|s| s.to_string()),
                    };
                    entries.insert(entry_name, entry);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_test() {
        let mut db = keepass::Database::new(Default::default());
        let mut group = Group::new("Group");
        let values = ["A", "B"];
        for val in values {
            let mut entry = keepass::db::Entry::new();
            entry.fields.insert(
                "Title".to_string(),
                keepass::db::Value::Unprotected(format!("title_{}", val)),
            );
            entry.fields.insert(
                "UserName".to_string(),
                keepass::db::Value::Unprotected(format!("username_{}", val)),
            );
            entry.fields.insert(
                "Password".to_string(),
                keepass::db::Value::Unprotected(format!("password_{}", val)),
            );

            entry.fields.insert(
                "URL".to_string(),
                keepass::db::Value::Unprotected(format!("url_{}", val)),
            );
            group.children.push(Node::Entry(entry));
        }

        db.root.children.push(Node::Group(group));

        let mut entries = HashMap::new();

        keepass_entries(&mut entries, &db.root, None);

        for key in values {
            let group = format!("Group/title_{}", key);
            let entry = entries.get(&group).unwrap();
            assert_eq!(entry.username, Some(format!("username_{}", key)));
            assert_eq!(entry.password, format!("password_{}", key));
            assert_eq!(entry.url, Some(format!("url_{}", key)));
        }
    }
}
