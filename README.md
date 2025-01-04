# page

Password manager with [age encryption](https://age-encryption.org/).

This is a fork of [passage](https://github.com/stchris/passage) by [strchris](https://github.com/stchris) since it is no longer maintained and I enjoyed using it. It was also a good excuse for me to start digging into rust. Note that I've renamed the tool `page`, given that the author of [age](https://age-encryption.org/) has released a popular alternative to the [password-store](https://www.passwordstore.org/), which is also called [passage](https://github.com/FiloSottile/passage).

Note that `page` is compatible with the password database file created by the original `passage`, so you can safely migrate to this tool.

## Installation

### Binaries and packages (preferred)

The [release page](https://github.com/deeuu/page/releases) includes binaries for Linux, mac OS and Windows.

### Build from source (for development)

With a rust toolchain present, you could do this (which makes sense if you want to contribute):

```bash
$ git clone https://github.com/deeuu/page

# Dependencies for Debian / Ubuntu
$ apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libdbus-1-dev

$ cargo install --path .
```

## Walkthrough

`page` creates an age-encrypted storage file (the password database), whose current default location depends on the OS family, for a given username `user`:

    Linux: `/home/user/.local/share/page/entries.toml.age`
    mac OS: `/Users/user/Library/Application Support/page/entries.toml.age`
    Windows: `C:\Users\user\AppData\Roaming\page\data\entries.toml.age`

You can create this file by running `page init` once. Check the path to the storage folder at any time with `page info`:

```bash
$ page info
Storage file: /home/deeuu/.local/share/page/entries.toml.age
```

Now let's create a new entry called `email`, with `$ page new email`:

```bash
Enter passphrase:
Password for 'email':
```

So here we are prompted for two things:

1. `Enter passphrase` is the secret we want to encrypt the password database with (this is used to enrypt the entries file, so applies to all passwords).
2. `Password for 'email'` is the password for the entry named `email` that we want to store.

Now `page list` should show one entry (`email`), which we can decrypt with either:

```bash
$ page show <entry> # the password gets copied to the clipboard
```

or

```bash
$ page show --on-screen <entry> # the password is printed to the console
```

`page` supports additional (but optional) attributes such as the username and url associated with the entry. For example:

```bash
$ page new <entry> --username <user> --url <url> # include username and url
$ page show <entry> --attribute username         # copy the username to the clipboard
```

## Hooks

`page` is able to call into [git-style hooks](https://git-scm.com/book/uz/v2/Customizing-Git-Git-Hooks) before or after certain events which affect the password database. A typical use case for hooks is if your password file is stored in version control and you want to automatically push/pull the changes when interacting with `page`.

To use hooks, place executable scripts, named after the hook you want to react on, inside the hooks folder (its path can be seen by running `page info`). These scripts are called and passed the event which triggered the hook as the first argument.

Existing hooks:

* `pre_load` (called before the password database gets loaded)
* `post_save` (called after the password database is updated)

These commands trigger hooks:

* `page new` (`pre_load`, `post_save` with event name `new_entry`)
* `page list` (`pre_load` with event name `list_entries`)
* `page show` (`pre_load` with event name `show_entry`)
* `page edit` (`pre_load`, `post_save` with event name `edit_entry`)
* `page remove` (`pre_load`, `post_save` with event name `remove_entry`)

Example hook scripts can be found [here](https://github.com/deeuu/page/tree/main/example_hooks).

## Keyring integration

If possible, `page` will try to store the passphrase of your database into the OS keyring. You can run `page keyring check` to see if this works. If you no longer want the password to be stored in the keyring run `page keyring forget`.

To skip the keyring integration, `page` takes a global flag `--no-keyring`.

## Usage

```bash
$ page

A password manager with age encryption

Usage: page [OPTIONS] <COMMAND>

Commands:
  init     Initialize the password store
  new      Add a new entry
  list     List all known entries
  show     Decrypt and show an entry
  edit     Edit an entry
  remove   Remove an entry
  info     Display status information
  keyring  Keyring related commands
  help     Print this message or the help of the given subcommand(s)

Options:
  -n, --no-keyring  Disable the keyring integration
  -h, --help        Print help
  -V, --version     Print version
```
