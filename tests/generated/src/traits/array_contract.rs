// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::storage::Mapping;
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TestStruct {
    struct_mapping: Mapping<u128, u128>,
    struct_f_array: [u8; 32],
    struct_d_array: Vec<u128>,
}

#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct NestedTestStruct {
    test_struct: TestStruct,
    uint_field: u8,
}


#[openbrush::wrapper]
pub type ArrayContractRef = dyn ArrayContract;

#[openbrush::trait_definition]
pub trait ArrayContract {}
