fn main() {
    let res = volo_build::ConfigBuilder::default().write().unwrap();
    println!("{:?}", res)
}
