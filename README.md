# Gmail checker for unread messages

### For i3status-rust

I use i3/Sway wm and want to have a notification for new emails in i3 status line.

### Preparation

You need to have a fresh version of **rust** and **cargo**.

Clone this repository and build release version.

```
git clone https://github.com/Crandel/rust_gmail_checker.git

cd rust_gmail_checker

cargo build --release
```

After building you could move the binary file inside your **PATH**.

```
mv target/release/rust_gmail  ~/.local/bin/rust_gmail
```

During first run it will fail and create **.email.json** file with this structure.

```json```
$ rust_gmail
A:0
```

[
    {
        "mail_type": "gmail",
        "account": "account_name",
        "short_alias":"A",
        "client_id": "<client_id>",
        "client_secret": "<client_secret>"
    }
]
```

Just edit this file.
You could use several gmail accounts to have a personal and work notifications.
Unread count is available as `dbus` message.
