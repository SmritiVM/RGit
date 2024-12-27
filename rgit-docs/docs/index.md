# RGit - Git in Rust

## Commands

* `rgit setup-config` - Setup the Global Configuration for your rgit.
* `rgit init` / `rgit init <directory_name>` - Initialize an rgit repository.
* `rgit add <file_path>` - Add a particular file to stage it for commits.
* `rgit commit <message>` - Commit the added changes.
* `rgit log` - View a log of all the commits.
* `rgit jump-to <commit_id>` - Jump to a specific commit to restore the working directory state

## Setup
To create the binary executable run `cargo build` and the build file will get stored in the target/debug directory.

Run `sudo cp target/release/rgit /usr/local/bin/rgit` to copy the binary executable to the local system.

Type `rgit <command_name>` from any directory to run the executable

## Project structure
Project structure

```
rgit/
├── src/
│   ├── main.rs     
│   └── components
|       └── configsetup.rs
|       └── init.rs
|       └── add.rs
|       └── commit.rs
|       └── log.rs
|       └── jumpto.rs
|       └── mod.rs
│   └── structures
|       └── index.rs
|       └── commit.rs
|       └── paths.rs
|       └── mod.rs
│   └── utils
|       └── hash_and_compress.rs
|       └── message_handler.rs
|       └── mod.rs    
└── target/
└── Cargo.lock
└── Cargo.toml

```

