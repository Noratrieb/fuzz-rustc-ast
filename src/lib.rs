use arbitrary::{Arbitrary, Unstructured};

#[derive(Debug)]
pub struct RustCode(pub String);

impl<'a> Arbitrary<'a> for RustCode {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let ast = syn::Item::arbitrary(u)?;

        let tokens = quote::quote! { #ast };

        Ok(RustCode(tokens.to_string()))
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (5, Some(10))
    }
}
