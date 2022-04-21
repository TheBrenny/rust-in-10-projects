#![allow(non_snake_case)]
use std::process::Command;

fn main() -> () {
    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("branches broke")
        .stdout;

    // My first major roadblocks were how to pass variables to functions
    // This shows off one of Rust's strongest features (but a new dev's weakest understanding):
    // Borrowing.
    // Rust is secure. This means that for some reason, we need to borrow variables
    // when passing them to a function. It's kinda confusing, and makes no sense,
    // but that's how it goes, I guess.

    // https://doc.rust-lang.org/std/vec/struct.Vec.html#slicing
    // What's actually happening here is a different phenomenon. Because Vectors are mutable,
    // we need to pass through an IMMUTABLE version. This is known as a 'slice' and is created
    // by prefixing the vector with a '&'.
    // The turbofish at the end (::<Vec<&str>>) is how we type the response of collect.
    // Here, the & in front of str has a loosely similar meaning: because str is a slice, the
    // & means to borrow or something like that...

    // I can write over variables? This is called shadowing! (and it's intentional)
    let output = String::from_utf8_lossy(&output);
    let branch: &str = {
        // this is a block, and luckily it's an expression! that means whatever is
        // returned from this block is the value of the variable "branch"
        let mut branchName: &str = "";
        for b in output.split("\n") {
            if b.starts_with("*") {
                branchName = b.trim_start_matches("* ");
            }
        }
        // Surprise! There's no return statement, but you'll notice the lack of a semicolon!
        // This is how rust "returns" variables. Imo, verbosity is better, because this can easily
        // look like a typo, but hey, a lot of things in this language are weird to start off with...
        branchName
    };
    println!("Using branch: {}", branch);

    // get the first remote for the repo
    let upstream = Command::new("git")
        .arg("remote")
        .output()
        .expect("remotes broke")
        .stdout;
    let upstream = String::from_utf8_lossy(&upstream);
    let upstream = upstream.split("\n").collect::<Vec<&str>>()[0];

    if upstream.len() == 0 {
        panic!("No upstream found");
    }

    println!("Using upstream: {}", upstream);

    // run a command and pipe the io to this proc's io
    let mut proc = Command::new("git");
    proc.args(["push", "--set-upstream", upstream, branch]);
    if let Ok(mut child) = proc.spawn() {
        let code = child.wait().expect("push cancelled");
        // exit with the error code from 'git push set upstream'
        std::process::exit(code.code().unwrap());
    } else {
        panic!("push broke");
    }
}
