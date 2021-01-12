fn main() {
    let dkp_path = std::env::var("DEVKITPRO").unwrap();
    let wut_path = match std::env::var("WUT_ROOT") {
        Ok(val) => val,
        Err(_) => format!("{}/wut", dkp_path)
    };
    println!("cargo:rustc-link-search=native={}/lib", wut_path);
    println!("cargo:rustc-link-search=native={}/share", wut_path);
}
