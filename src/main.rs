
fn main() {
    let s = String::from("/Users/hellomac/RustApp/loader/target/debug/loader");
    // let rs = s.strip_suffix("/loader").unwrap_or("No");
    if let Some(r) = s.rfind('/') {
        if let Some(x) = s.get(0..r) {
            println!("{}",x);
        }
    }
    // println!("{}",rs);
}
