use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TupleField)]
pub fn tuplefield_proc(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_tuplespace(&ast)
}

fn impl_tuplespace(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[typetag::serde]
        impl ::rspaces::TupleField for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn box_clone(&self) -> Box<dyn TupleField> {
                Box::new((*self).clone())
            }
            fn query(&self, element: &Box<dyn TupleField>, matching: &TemplateType) -> bool {
                match matching {
                    TemplateType::Actual => match (*element).as_any().downcast_ref::<Self>() {
                        Some(e) => *self == *e,
                        None => false,
                    },
                    TemplateType::Formal => match (*element).as_any().downcast_ref::<Self>() {
                        Some(_) => true,
                        None => false,
                    },
                }
            }
        }
    };
    gen.into()
}
