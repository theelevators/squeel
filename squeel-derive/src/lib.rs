use syn::DeriveInput;
#[proc_macro_derive(Entity, attributes(datatype))]
pub fn data_types_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    data_types_derive_macro2(item.into()).unwrap().into()
}

fn extract_fields(ast: &mut DeriveInput) -> deluxe::Result<Vec<String>>{
    let mut fields: Vec<String> = Vec::new();

    if let syn::Data::Struct(s) =  &mut ast.data{
        for field in s.fields.iter_mut(){


            let field_name = field.ident.as_ref().unwrap().to_string();
            
            fields.push(field_name);
           
        }
    }
    return  Ok(fields);
}


fn data_types_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;

    let fields:Vec<String> = extract_fields(&mut ast)?;
    
    let ident = &ast.ident;
    let ident_str = ident.to_string();

    let (impl_generic, type_generics, where_clause) = ast.generics.split_for_impl();

    return Ok(quote::quote!(

        impl #impl_generic Entity for #ident #type_generics #where_clause {

            fn as_table() -> String {
                let fields = [#(#fields),*].join(",");
                format!("SELECT {} FROM {} FOR JSON PATH;",fields,#ident_str)
            }
        }
    ));
}


