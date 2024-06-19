#[link(name = "compiled_code", kind = "static")]
extern "sysv64" {
    fn start_here() -> i64;
}

fn main() {
    let output = unsafe { start_here() };
    println!("Assembly code returned: {}", output);
}
