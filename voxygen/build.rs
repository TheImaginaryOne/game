use std::{
    env,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    // @todo Error checking

    // Git hash
    let output_hash = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output_hash.stdout).unwrap();

    // Git time
    let output_time = Command::new("git")
        .args(&["log", "-1", "--pretty=format:%ct"])
        .output()
        .unwrap();
    let git_time = String::from_utf8(output_time.stdout).unwrap();

    // Profile
    let profile = env::var("PROFILE").unwrap();

    // Build time
    let build_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("-1").as_secs();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash); // Store as GIT_HASH env variable
    println!("cargo:rustc-env=GIT_TIME={}", git_time); // Store as GIT_TIME env variable
    println!("cargo:rustc-env=PROFILE={}", profile); // Store as PROFILE env variable
    println!("cargo:rustc-env=BUILD_TIME={:?}", build_time); // Store as BUILD_TIME env variable
}
