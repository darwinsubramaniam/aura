fn main() {
    // 1. Load the .env file variables into the current environment
    if let Ok(path) = dotenvy::dotenv() {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // 2. Iterate over specific variables you want to expose to the compiler
    //    WARNING: Do not expose everything blindly to avoid leaking secrets!
    let key = "ENV";
    if let Ok(value) = std::env::var(key) {
        // This magic line tells Cargo: "Pass this variable to the compiler"
        println!("cargo:rustc-env={}={}", key, value);
    }

    tauri_build::build()
}
