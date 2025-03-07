use std::process::Command;

fn main() {
    println!("🚀 Starting mdBook deployent...");

    // Step 1: Install mdBook
    let _ = Command::new("cargo")
        .arg(&["install", "mdbook"])
        .status()
        .expect("Failed to install mdBook");

    // Step 2: Build the book
    let _ = Command::new("mdbook")
        .arg("build")
        .status()
        .expect("Failed to build the book");

    // Step 3: Deploy the 'gh-pages' branch
    let _ = Command::new("git")
        .args(&["add", "."])
        .status()
        .expect("Git add failed");

    let _ = Command::new("git")
        .args(&["commit", "-m", "Deploying book update"])
        .status()
        .expect("Git commit failed");

    let _ = Command::new("git")
        .args(&["push", "origin", "gh-pages"])
        .status()
        .expect("Git push failed");

    println!("🎉 Deployment complete!");       

}