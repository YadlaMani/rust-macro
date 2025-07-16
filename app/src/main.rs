use std::fmt::Error;
use serialize_macro::{SerializeNumberStruct,DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct, Debug)]
struct Swap{
    value1: u32,
    value2: u32,
    value3:u16,
    name1: String,
   
    
}
fn main(){
    let swap=Swap{
        value1: 42,
        value2: 84,
        value3: 16,
        name1: "Alice".to_string(),
       
    };
    let serialized=swap.serialize();
    println!("Serialized Swap: {:?}", serialized);

    let deserialized_swap=Swap::deserialize(&serialized).unwrap();
    println!("Deserialized Swap: {:?}", deserialized_swap);
}