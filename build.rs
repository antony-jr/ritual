use std::env;

fn get_and_set_build_var(contents: Vec<&str>) {
    for i in contents.iter() {
        match env::var(i) {
            Ok(value) => {
                println!("cargo:rustc-env={}={}", i, value);
            },
            _ => {
                let default = if i.contains("VERSION") {
                    "dev-build"
                } else {
                    "none"
                };
                println!("cargo:rustc-env={}={}", i, default);
            }
        };
    }
}

fn main() {
    get_and_set_build_var(
        vec![
        "RITUAL_VERSION",
        "RITUAL_GIT_COMMIT",
        "RITUAL_BUILD_NO",
        "RITUAL_BUILD_DATE"]);
}
