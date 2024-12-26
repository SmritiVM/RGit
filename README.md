# RGitS

## Getting Started 

Project structure

```
wegit/
├── src/
│   ├── main.rs     
│   └── components
│   └── structures
│   └── utils    
└── Cargo.toml       // Project metadata and dependencies
```

Make the script executable

To make the Rust binary executable, you compile it using:

``` cargo build --release ```

The resulting executable will be located in ./target/release/wyag.

If you'd like it to be easily runnable from anywhere, you can copy it to a directory in your $PATH:

``` cp target/release/wegit /usr/local/bin/wegit```

Now the binary executable has been created, type ```wegit``` in the terminal to run it 

### Insights
This can also be run using ```cargo run```

## Adding command-line arguments

1.Install the clap library for argument parsing.

Add the following to Cargo.toml:
```
[dependencies]
clap = { version = "4.3", features = ["derive"] }
```
2.    Update src/main.rs to handle command-line arguments like the Python version.

Use ```clap``` to parse arguments and subcommands
Use ```enums``` to match subcommands, similar to Python's ```match-case```

The version attribute of the #[command] macro is what is displayed when ```<command name> --version``` is entered. Similarly the about attribute is mapped to ```--help```

To execute the command, build again using ```cargo build --release``` and run using ```/target/release/wegit <command name>```

## Git Init
Git Init is used to initialize a directory as a git repository. The sequence of actions when ```wegit init``` is run is as follows:

1. A new directory (folder) is created with the directory name passed as an argument. If no directory name is passed, the current directory name is taken as the directory name

2. A ```.wegit``` folder in the directory

3. A ```config.toml``` file in the ```.wegit``` that contains the configurations.

4. The local config.toml should derive the [user] field from the global config.toml. This is set up using configsetup.

6. A `file_objects` and `index_objects` folder to store the compressed contents of the file when git add is performed and git commit are performed respectively.

### Commands/Operators Learnt

Creation of a new directory happens with the fs::create_dir_all function from the std::fs library

```eprintln!``` is especially used to print errors. It will be sent to the Standard Error stream rather than the standard output stream

```to_string_pretty``` generates TOML data as a String with extra whitespace (indentation) to improve the readability of the resulting TOML file

```.and_then(|user_section| { ... })```
.and_then() is a combinator on Option<T>. It takes a closure (the code inside the curly braces) and applies it to the value inside the Option if the option is Some(value).

If the option is None (i.e., if the "user" section doesn't exist in the TOML), the closure is never executed, and None is returned.

If "user" is found (i.e., the value inside the Option is Some), it passes user_section (the value associated with "user") to the closure.


```.ok()``` converts a Result into an Option. If to_string() succeeds, it returns Some(String). If it fails, it returns None.

```?``` is the "try" operator, which returns None immediately if to_string() fails (i.e., if it returns Err).

`::` helps you refer to and access elements from various namespaces, such as modules, types, methods, constants, and enums. It clarifies where each item comes from and makes the code more organized and structured.


In Rust, an `enum` (short for "enumeration") is a type that can represent one of several possible variants. Enums are used to define a type that can take on different values, each representing a different state or variant. Each variant can be simple, like a single value, or more complex, containing associated data.

### Insights
- `?` vs `expect`

| Feature                | `?` Operator                                        | `expect` Method                              |
|------------------------|-----------------------------------------------------|----------------------------------------------|
| **Error Handling**      | Propagates errors to the caller                     | Causes a panic and terminates the program    |
| **Return Type**         | Requires the function to return `Result<T, E>` or `Option<T>` | Requires the function to return `Result<T, E>` or `Option<T>` |
| **Use Case**            | Used when you want to propagate errors without halting execution | Used when you want to ensure success and panic on failure |
| **Error Message**       | No custom error message, the error is returned as-is | Custom error message can be provided when the panic occurs |



- While parsing toml files, ChatGPT suggests using ```use serde::Serialize;```. The rust compiler throws the following error:
```
error: cannot find derive macro `Serialize` in this scope
 --> src/libwegit.rs:8:10
  |
8 | #[derive(Serialize)]
  |          ^^^^^^^^^
  |
note: `Serialize` is imported here, but it is only a trait, without a derive macro
 --> src/libwegit.rs:6:5
  |
6 | use serde::Serialize;
  |     ^^^^^^^^^^^^^^^^
help: consider importing this derive macro
  |
1 + use serde_derive::Serialize;

```
```use serde_derive::Serialize``` is the macro to be imported.

## Git Add
Steps in git add

1. Addition of ```<file path> <file hash>``` to the index.txt file in # wegitS
2. Creation of a compressed version of the file in the ```file_objects``` folder of the .wegit

Git stores these objects using SHA-1 hashing and organizes them in subdirectories based on the first two characters of the hash. Specifically:

The content of the file is hashed to generate a SHA-1 object identifier.
The object is stored as a file in .git/objects/`<first two characters of the SHA-1 hash>/<remaining characters of the SHA-1 hash>.`

