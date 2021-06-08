extern crate gcc;

fn main() {
    let current_dir = std::env::current_dir().expect("Current directory is invalid. Do you have sufficient permissions.");
    let current_dir = current_dir.into_os_string().into_string().unwrap();

    println!("cargo:rustc-link-lib=static=zpie");
    println!("cargo:rustc-link-lib=static=mclbn384_256");
    println!("cargo:rustc-link-lib=static=mcl");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-search={}/libraries", current_dir);
}
