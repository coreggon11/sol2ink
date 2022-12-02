#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (token/ERC1155/ERC1155.sol)
#[openbrush::contract]
pub mod arrays {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::Storage,
    };
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Debug, Encode, Decode, PartialEq)]
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

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Data {
        pub storage_mapping: Mapping<u128, u128>,
        pub storage_f_array: [u8; 32],
        pub storage_d_array: Vec<u128>,
        pub storage_d_struct_array: Vec<NestedTestStruct>,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Arrays {
        #[storage_field]
        data: Data,
    }

    impl Arrays {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

        fn _work_with_arrays(&self, element: u128, d_array: Vec<u8>) -> Result<Vec<u128>, Error> {
            // fn parameters (error with f_array)
            d_array[4] = element;
            // declaration (error with f_array)
            let function_d_array: Vec<u128> = vec![u128::default(); 1];
            function_d_array[0] = element;
            // assign value
            self.data.storage_f_array[0] = 0;
            self.data.storage_d_array[2] = 2;
            self.data.storage_mapping.insert(&1, &(element));
            // assign array type
            function_d_array[1] = self.data.storage_f_array[0];
            function_d_array[2] = self.data.storage_d_array[2];
            element = self.data.storage_mapping.get(&1).unwrap_or_default();
            // nested array
            self.data.storage_d_struct_array[1]
                .test_struct
                .struct_f_array[2] = 2;
            self.data.storage_d_struct_array[3]
                .test_struct
                .struct_d_array[3] = 3;
            self.data.storage_d_struct_array[2]
                .test_struct
                .struct_mapping
                .get(&2)
                .unwrap_or_default() = 1;
            element = self.data.storage_d_struct_array[1]
                .test_struct
                .struct_f_array[2];
            element = self.data.storage_d_struct_array[3]
                .test_struct
                .struct_d_array[3];
            element = self.data.storage_d_struct_array[2]
                .test_struct
                .struct_mapping
                .get(&2)
                .unwrap_or_default();
            // struct fields
            let test_struct: TestStruct;
            test_struct.struct_f_array[2] = element;
            test_struct.struct_d_array[3] = 3;
            test_struct.struct_mapping.insert(&10, &(element));
            // nested struct fields
            let nested_test_struct: NestedTestStruct;
            nested_test_struct.test_struct.struct_f_array[2] = element;
            nested_test_struct.test_struct.struct_d_array[3] = 3;
            nested_test_struct
                .test_struct
                .struct_mapping
                .insert(&10, &(element));
            // assign struct field
            function_d_array[1] = test_struct.struct_f_array[2];
            function_d_array[2] = nested_test_struct.test_struct.struct_d_array[3];
            element = test_struct.struct_mapping.get(&10).unwrap_or_default();
            // // push
            // function_d_array.push(1);
            // storage_d_array.push(1);
            //
            // // pop
            // function_d_array.pop();
            // storage_d_array.pop();
            return Ok(function_d_array)
        }

    }
}
