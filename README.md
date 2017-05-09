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
alert run my_long_running_command
```

This will play different sounds for success and failure.

Right now, we only support Superblock, LLC's [Pushover][] system, which provides
excellent, hard-to-miss notifications for iOS, Android and web browsers.
(This application has no relation to Superblock, LLC. We just use their API.)

[Pushover]: https://pushover.net/

## Wish list

Some features which we want to add:

```sh
# Periodically poll commands and read their output.
alert watch -n 5 -s 'myjob.*failed' -f 'myjob.*success' \
    --timout 10m \
    pachctl list-jobs
```

We'd also list to add:

- Support for desktop notifications on OS X and Linux.
- Support for free phone notifications using [notify-cli][]'s API.  These
  are less portable than Pushover's notifications and they have fewer options
  for playing loud and obnoxious sounds, but they're free.
- Maybe some sort of mode for watching to see when a host has finished booting?

[notify-cli]: https://github.com/mashlol/notify

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

