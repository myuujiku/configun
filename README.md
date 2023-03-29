# Configun (WIP)

Shoot all your config with one gun.


## About

Configun aims to provide a fast and simple way to manage configuration files on multiple systems via a CLI.
All configuration is stored in one Configun repository which holds information about the repository itself, compressed and optionally
encrypted configuration packages called bullets (.bpk), and information about how those bullets are distributed on the target system.


## State

Configun is currently in the concept phase and an MVP is not ready yet.


## Motivation

Directories like `$HOME` and `$XDG_CONFIG_HOME` are often cluttered with automatically generated files that make it harder to maintain configuration that needs to be modified manually.
The divide of applications following the XDG standard complicates this further. 

It is possible to use Git as a configuration management solution, but that comes with it's own share of problems. Not every system has the same requirements. A server won't need configuration for GUI applications and a PC won't need Nginx. If the configuration is shared over the internet there might some information that shouldn't be available to everyone, like passwords or SSH keys.

Configun tries to address those issues with a modular configuration system tailor-made for configuration management.



## Development

You need to have [Rust](https://www.rust-lang.org) installed to compile Configun. Configun uses Rust's default package manager
[Cargo](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html) for dependency management and building.
