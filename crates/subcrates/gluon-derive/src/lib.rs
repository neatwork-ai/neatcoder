use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, __private::TokenStream};

#[proc_macro_derive(Sample)]
pub fn derive_sample(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let sample_impl = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let field_names = fields.named.iter().map(|field| &field.ident);
            let sample_values = fields
                .named
                .iter()
                .map(|field| generate_sample_value(&field.ty));

            quote! {
                impl Sample for #struct_name {
                    fn sample() -> Self {
                        Self {
                            #(#field_names: #sample_values),*
                        }
                    }
                }
            }
        }
        _ => panic!("Example derive only supports named fields in structs"),
    };

    TokenStream::from(sample_impl)
}

fn generate_sample_value(ty: &syn::Type) -> quote::__private::TokenStream {
    if let syn::Type::Path(type_path) = ty {
        let type_name = type_path.path.segments.last().unwrap().ident.to_string();

        match type_name.as_str() {
            "String" => quote! { String::from("example") },
            "u8" => quote! { 0u8 },
            "u16" => quote! { 0u16 },
            "u32" => quote! { 0u32 },
            "u64" => quote! { 0u64 },
            "i8" => quote! { 0i8 },
            "i16" => quote! { 0i16 },
            "i32" => quote! { 0i32 },
            "i64" => quote! { 0i64 },
            "f32" => quote! { 0.0f32 },
            "f64" => quote! { 0.0f64 },
            _ => quote! { unimplemented!("Unsupported field type: {}", #type_name) },
        }
    } else {
        quote! { unimplemented!("Unsupported field type") }
    }
}
