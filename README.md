# Project Generator CLI

This CLI tool quickly creates project scaffolds for frameworks like Symfony, Flask, Django, and Rust.

## Features

- **Fast command checks:** Uses the `which` crate to quickly check for required tools.
- **Non-interactive mode:** Use the `--no-prompt` flag to run without any manual confirmations.
- **Fallbacks:** Creates a basic directory structure if a required CLI tool is missing.

## Installation

1. Install [Rust](https://rustup.rs/).
2. Clone this repository:
   ```bash
   git clone https://github.com/xmlxss/project_generator_cli.git
3. Build the project in release mode:
   ```bash
   cargo build --release

## Usage
- **Symfony**
    ```bash
    project_generator_cli symfony my_project
- **Flask**
    ```bash
    project_generator_cli flask my_project
- **Django**
    ```bash
    project_generator_cli django my_project
- **Rust**
    ```bash
    project_generator_cli rust my_project

- **To Run without interactive prompts use `--no-promt` arg**
    ```bash
    project_generator_cli --no-prompt flask my_project