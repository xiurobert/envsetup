use envsetup::run;

fn main() {
    println!("envsetup v{}", env!("CARGO_PKG_VERSION"));
    // todo: support for specifying a github repo, and pulling the information from there
    run("envsetup.yml");
}
