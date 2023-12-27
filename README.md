# Introduction

This project is a simple rust version of AutoGPT. It has three agents and can help users to generate back-end code for requirements in natural language.

This project is a simple learning project of a online course and just for future personal reference.

# Usage

The setup of rust environment is required. Please refer to [rustup](https://rustup.rs/) for more details.

After setting up the rust environment, simple run `cargo build` to build the project.

Then run `cargo run` to start the project.

Then you can interact with the agents in the terminal.

# Agents

There are three agents in this project. They are managing agent, solution architect and back-end developer.

# Key Points

## Mod

**Declaring modules**

For example, declare a module named "garden" with `mod graden`. The compiler will look for this module in these places:

- Inline, `mod garden { ... }`
- In the file *src/garden.rs*
- In the file *src/garden/mod.rs*

**Declaring submodules**

For example, declare a submodule named "vegetable" in the module "garden" with `mod vegetable` in file *src/garden.rs*. The compiler will look for this submodule in these places:

- Inline, `mod vegetable { ... }`
- In the file *src/garden/vegetable.rs*
- In the file *src/garden/vegetable/mod.rs*

**NOTE**

1. If we only create a file *src/garden.rs* or *src/garden/mod.rs*, the compiler will not include this module (declared or brought into scope) unless we use `mod garden;` in the file *src/main.rs* or in modules which are already included in the file *src/main.rs*.
2. `mod xxx;` is not needed for external modules, since they are already be specified in the file *Cargo.toml*.
3. `use` is not used for bringing modules into scope, it is used for creating shortcuts to items to reduce repetition of long paths.
4. In Rust 2015 edition, we need to use `extern crate` to bring external crates into scope, for example `extern crate rand;`. While in Rust 2018 edition, we can use `use` to bring external crates into scope, for example `use rand;`.

## Command Line Tool

Standard IO library is enough for handle user input and output. While `crossterm` can be used to set the color of the text in the terminal.

```rust
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespaces and return
    user_response.trim().to_string()
}
```

