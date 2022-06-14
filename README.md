# Thinkpad-docking

A simple daemon that handles a small subset of ACPID events related to thinkpad laptop.
This program is similar to [dockd](https://github.com/libthinkpad/dockd).
It listens on the ACPID socket for events and then applies the user-configured screen configuration.
We use [quickrandr](https://crates.io/crates/quickrandr) to interface with the `xrandr` program.

This project is a Rust progamming language exercise.

If you're interested in the funcitonality only then it is best to use what the acpid daemon provides already via shell script hooks. See `man 8 acpid` for details.
