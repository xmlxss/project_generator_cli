use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use which::which;

#[derive(Parser)]
#[command(name = "Project Generator")]
#[command(about = "Project to create projects", long_about = None)]
struct Cli {
    #[arg(long)]
    no_prompt: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Symfony {
        project: String,
    },
    Flask {
        project: String,
    },
    Django {
        project: String,
    },
    Rust {
        project: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Symfony { project } => create_symfony_project(project, cli.no_prompt)?,
        Commands::Flask { project } => create_flask_project(project, cli.no_prompt)?,
        Commands::Django { project } => create_django_project(project, cli.no_prompt)?,
        Commands::Rust { project } => create_rust_project(project)?,
    }
    Ok(())
}

fn command_exists(command: &str) -> bool {
    which(command).is_ok()
}

fn prompt_yes_no(question: &str, no_prompt: bool) -> bool {
    if no_prompt {
        return true;
    }
    use dialoguer::Confirm;
    Confirm::new()
        .with_prompt(question)
        .default(true)
        .interact()
        .unwrap_or(false)
}

fn create_symfony_project(project: &str, no_prompt: bool) -> Result<()> {
    println!("Creating Symfony PHP project for: {}", project);
    if command_exists("symfony") {
        println!("Found Symfony CLI. Using 'symfony new'.");
        let status = Command::new("symfony")
            .args(&["new", project])
            .status()?;
        if !status.success() {
            return Err(anyhow!("Failed to create Symfony project with Symfony CLI."));
        }
    } else {
        println!("Symfony CLI not found.");
        if prompt_yes_no("Symfony CLI is missing. Create directory structure manually as fallback?", no_prompt) {
            let base = Path::new(project);
            let dirs = ["config", "public", "src", "templates", "var", "vendor"];
            for dir in &dirs {
                let path = base.join(dir);
                fs::create_dir_all(&path)
                    .map_err(|e| anyhow!("Failed to create directory {:?}: {}", path, e))?;
            }
            let index_path = base.join("public").join("index.php");
            fs::write(&index_path, "<?php\n// Symfony front controller placeholder\n")
                .map_err(|e| anyhow!("Failed to create file {:?}: {}", index_path, e))?;
            println!("Fallback Symfony project structure created successfully!");
        } else {
            println!("Please install the Symfony CLI from https://symfony.com/download and try again.");
        }
    }
    Ok(())
}

fn create_flask_project(project: &str, no_prompt: bool) -> Result<()> {
    println!("Creating Python Flask project for: {}", project);
    if !command_exists("python") && !command_exists("python3") {
        println!("Python was not found on your system.");
        if !prompt_yes_no("Abort and install Python first?", no_prompt) {
            return Err(anyhow!("Python is required for Flask projects."));
        }
    }
    let base = Path::new(project);
    let dirs = ["app", "venv", "static", "templates"];
    for dir in &dirs {
        let path = base.join(dir);
        fs::create_dir_all(&path)
            .map_err(|e| anyhow!("Failed to create directory {:?}: {}", path, e))?;
    }
    let app_file = base.join("app").join("app.py");
    let app_content = r#"from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello():
    return "Here we go again!"

if __name__ == '__main__':
    app.run(debug=True)
"#;
    fs::write(&app_file, app_content)
        .map_err(|e| anyhow!("Failed to create file {:?}: {}", app_file, e))?;

    println!("Setting up Python virtual environment (ensure Python is installed)...");
    let venv_dir = base.join("venv");
    let python_cmd = if command_exists("python") { "python" } else { "python3" };
    let status = Command::new(python_cmd)
        .args(&["-m", "venv", venv_dir.to_str().unwrap()])
        .status();

    match status {
        Ok(s) if s.success() => println!("Virtual environment created successfully!"),
        _ => {
            println!("Failed to create virtual environment.");
            if prompt_yes_no("Would you like to try again manually?", no_prompt) {
                println!("Please create the virtual environment using 'python -m venv venv' in your project directory.");
            }
        }
    }
    println!("Flask project scaffold created successfully!");
    Ok(())
}

fn create_django_project(project: &str, no_prompt: bool) -> Result<()> {
    println!("Creating Django project for: {}", project);
    if command_exists("django-admin") {
        println!("Found django-admin. Using 'django-admin startproject'.");
        let status = Command::new("django-admin")
            .args(&["startproject", project, project])
            .status()?;
        if !status.success() {
            return Err(anyhow!("Failed to create Django project with django-admin."));
        }
    } else {
        println!("django-admin not found.");
        if prompt_yes_no("django-admin is missing. Create basic scaffold manually as fallback?", no_prompt) {
            let base = Path::new(project);
            let dirs = ["project", "app", "venv"];
            for dir in &dirs {
                let path = base.join(dir);
                fs::create_dir_all(&path)
                    .map_err(|e| anyhow!("Failed to create directory {:?}: {}", path, e))?;
            }
            let manage_py = base.join("manage.py");
            let manage_content = r#"#!/usr/bin/env python
import os
import sys

if __name__ == '__main__':
    os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'project.settings')
    try:
        from django.core.management import execute_from_command_line
    except ImportError as exc:
        raise ImportError("Couldn't import Django.") from exc
    execute_from_command_line(sys.argv)
"#;
            fs::write(&manage_py, manage_content)
                .map_err(|e| anyhow!("Failed to create file {:?}: {}", manage_py, e))?;
            let settings_file = base.join("project").join("settings.py");
            let settings_content = r#"SECRET_KEY = 'your-secret-key'
DEBUG = True
ALLOWED_HOSTS = []
INSTALLED_APPS = [
    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',
    'app',
]
MIDDLEWARE = [
    'django.middleware.security.SecurityMiddleware',
    'django.contrib.sessions.middleware.SessionMiddleware',
    'django.middleware.common.CommonMiddleware',
]
ROOT_URLCONF = 'project.urls'
"#;
            fs::write(&settings_file, settings_content)
                .map_err(|e| anyhow!("Failed to create file {:?}: {}", settings_file, e))?;
            println!("Fallback Django scaffold created successfully!");
        } else {
            println!("Please install Django (pip install Django) to use the standard generator.");
        }
    }
    println!("Setting up Python virtual environment (ensure Python is installed)...");
    let venv_dir = Path::new(project).join("venv");
    let python_cmd = if command_exists("python") { "python" } else { "python3" };
    let status = Command::new(python_cmd)
        .args(&["-m", "venv", venv_dir.to_str().unwrap()])
        .status();
    match status {
        Ok(s) if s.success() => println!("Virtual environment created successfully!"),
        _ => println!("Failed to create virtual environment. Please create it manually."),
    }
    Ok(())
}

fn create_rust_project(project: &str) -> Result<()> {
    println!("Creating Rust project for: {}", project);
    if !command_exists("cargo") {
        return Err(anyhow!("Cargo was not found on your system. Please install Rust (and Cargo) from https://rustup.rs."));
    }
    let status = Command::new("cargo")
        .args(&["new", project])
        .status()?;
    if status.success() {
        println!("Rust project created successfully!");
    } else {
        return Err(anyhow!("Failed to create Rust project using Cargo."));
    }
    Ok(())
}
