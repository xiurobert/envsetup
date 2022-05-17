use envsetup::run;

fn main() {
    println!("envsetup v{}", env!("CARGO_PKG_VERSION"));
    run("envsetup.yml");
}
