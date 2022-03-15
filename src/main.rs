use anyhow::Context;
use arbitrary::Arbitrary;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Cargo.toml");
    let data = std::fs::read(&path).context(path.display().to_string())?;

    // %%% G% GGGG   �
    let mut unstructured = arbitrary::Unstructured::new(&data);

    let ast = syn::Item::arbitrary_take_rest(unstructured).context("from arbitrary")?;

    let tokens = quote::quote! { #ast };

    println!("{tokens}");

    Ok(())
}
