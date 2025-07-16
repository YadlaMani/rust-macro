use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields};

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(input:TokenStream)->TokenStream{
    let ast:DeriveInput=syn::parse(input).unwrap();
    let name=&ast.ident;
    let serialize_fields=match &ast.data{
        Data::Struct(data_struct)=>{
            match &data_struct.fields{
                Fields::Named(fields)=>{
                    let field_serializations=fields.named.iter().map(|field|{
                        let field_name=&field.ident;
                        let ty=&field.ty;
                        let ty_str=quote!(#ty).to_string();
                       
                            if ty_str=="String"{
                                quote!{
                                    let str_bytes=&self.#field_name.as_bytes();
                                    let len=str_bytes.len() as u32;
                                    result.extend_from_slice(&len.to_be_bytes());
                                    result.extend_from_slice(str_bytes);
                                }
                            }else{
                                quote!{
                                    result.extend_from_slice(&self.#field_name.to_be_bytes());
                                }
                            }
                                
                                    
                              
                                    
                                
                            
                           
                        
                    });
                    quote!{
                         #(#field_serializations)*
                    }
                }
                _=>panic!("Only named fields are supported for serialization"),
            }
        }
        _=>panic!("Only structs are supported")
    };
    let generate=quote!{
        impl Serialize for #name{
            fn serialize(&self)->Vec<u8>{
                let mut result=Vec::new();
                #serialize_fields
                result
            }
        }
    };
    generate.into()
}
#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input:TokenStream)->TokenStream{
    let ast:DeriveInput=syn::parse(input).unwrap();
    let name=&ast.ident;
    let (deserialize_fields,field_assignments,total_size)=match &ast.data{
        Data::Struct(data_struct)=>{
            match &data_struct.fields{
            Fields::Named(fields)=>{
                let mut offset:usize=0;
                let mut field_deserializations=Vec::new();
                let mut field_assignments=Vec::new();
                for field in &fields.named{
                    let field_name=&field.ident;
                    let ty=&field.ty;
                    let (size,parse_expr,is_string)=match quote!(#ty).to_string().as_str(){
                        "u8"=>(1,quote!{u8::from_be_bytes},false),
                        "u16"=>(2,quote!{u16::from_be_bytes},false),
                        "u32"=>(4,quote!{u32::from_be_bytes},false),
                        "u64"=>(8,quote!{u64::from_be_bytes},false),
                        "i8"=>(1,quote!{i8::from_be_bytes},false),
                        "i16"=>(2,quote!{i16::from_be_bytes},false),
                        "i32"=>(4,quote!{i32::from_be_bytes},false),
                        "i64"=>(8,quote!{i64::from_be_bytes},false),
                        "String"=>(0,quote!{},true),
                        _=>panic!("Unsupported field"),

                    };
                    if is_string {
   


    field_deserializations.push(quote! {
        let len_bytes: [u8; 2] = data[offset..offset+2].try_into().map_err(|_| Error)?;
let str_len = u16::from_be_bytes(len_bytes) as usize;
offset += 2;

let #field_name = std::str::from_utf8(&data[offset..offset + str_len])
    .map_err(|_| Error)?
    .to_string();
offset += str_len;
    });

    field_assignments.push(quote! {
        #field_name
    });

    offset += 2; 
    
    field_deserializations.push(quote! {
        offset += str_len;
    });
}
                    else{
                        let start_offset=offset;
                    let end_offset=start_offset+size;
                    

                  
                    field_deserializations.push(quote!{
                        let #field_name={
                            let bytes:[u8;#size]=data[#start_offset..#end_offset].try_into().map_err(|_|Error)?;
                            #parse_expr(bytes)
                        };
                    });
                    field_assignments.push(quote!{
                        #field_name
                    });
                    offset+=size;
                    }
                    
                }
                (field_deserializations, field_assignments, offset)
            }
            _=>panic!("Only named fields are supported"),
        }
    }
    _=>panic!("Only structs are supported"),
};
let generate=quote!{
    impl Deserialize for #name{
        fn deserialize(data:&[u8])->Result<Self,Error>{
            if data.len()<#total_size{
                return Err(Error);
            }
            let mut offset:usize=0;
            #(#deserialize_fields)*
            Ok(#name{
                #(#field_assignments),*
            })
        }
    }
};
generate.into()
    
}
