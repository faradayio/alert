# `alert`: Notify your desktop or mobile device when a job completes

**WORK IN PROGRESS.** This is under heavy development the week of 8 May 2017.

At Faraday, we have a lot of long-running processes, and we need an easy way to
know when they complete. The `alert` command is intended to make it easy to know
when things succeed or fail:

```sh
# Supply a pushover.net application token.  We'll make it easy to create these
# soon.
export PUSHOVER_TOKEN=...
# Supply your pushover.net user key.
export PUSHOVER_USER=...

# Run a command and report what happens.
alert run my_long_running_command

# Run a command repeatedly, looking for outputs that indicate success or
# failure.
alert watch -s 'myjob.*failed' -f 'myjob.*success' --timout 300 \
    pachctl list-jobs
```

This will play different sounds for success and failure.

Right now, we support Superblock, LLC's [Pushover][] system, which provides
excellent, hard-to-miss notifications for iOS, Android and web browsers.
(This application has no relation to Superblock, LLC. We just use their
API.)

We also have support for desktop notifiations and the open
source [Notify][] app, but the CLI for configurating them isn't finished
yet. To use desktop notifications, try:

```sh
export ALERT_NOTIFIER=desktop
```

[Pushover]: https://pushover.net/
[Notify]: https://mashlol.github.io/notify/

## Wish list

Some features which we want to add:

- A command-line interface for configuring notification backends easily.
- Terminal support for `alert watch`.
- Maybe some sort of mode for watching to see when a host has finished booting?

## Installing

On Linux, you'll need to install the `dbus-1` dev packages.

```sh
# Ubuntu, etc.
sudo apt install libdbus-1-dev
```

Once this is done, install Rust and `alert`:

```sh
curl https://sh.rustup.rs -sSf | sh
cargo install alert
```
