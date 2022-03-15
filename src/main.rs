use arbitrary::Arbitrary;
use std::path::PathBuf;

fn main() {
    let data = std::fs::read(PathBuf::from(file!()).parent().unwrap().join("Cargo.toml")).unwrap();
    let mut unstructured = arbitrary::Unstructured::new(&data);

    let ast = syn::Item::arbitrary(&mut unstructured);

    println!("{ast}");
}
