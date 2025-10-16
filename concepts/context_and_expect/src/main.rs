/// Concept: Context & Expect
use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    // rust_expect();
    let context_result = anyhow_context()?;
    println!("Anyhow context: {:?}", context_result);
    Ok(())
}

fn rust_expect() {
    let content = fs::read_to_string("hello.txt").expect("Failed to read hello.txt");

    println!("Expect:");
    println!("Content: {}", content);
}

fn anyhow_context() -> Result<String> {
    let content = fs::read_to_string("hello.txt").context("Failed to read hello.txt")?;

    println!("Context:");
    println!("Content: {}", content);

    Ok(content)
}
