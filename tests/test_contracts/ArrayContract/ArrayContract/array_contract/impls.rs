// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::Storage,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub storage_mapping: Mapping<u128, u128>,
    pub storage_f_array: [u128; 13],
    pub storage_d_array: Vec<u128>,
    pub storage_d_struct_array: Vec<NestedTestStruct>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ArrayContract for T {}

pub trait Internal {
    fn _work_with_arrays(
        &self,
        element: u128,
        f_array: [u8; 13],
        d_array: Vec<u8>,
    ) -> Result<Vec<u128>, Error>;

}

impl<T: Storage<Data>> Internal for T {
    default fn _work_with_arrays(
        &self,
        element: u128,
        f_array: [u8; 13],
        d_array: Vec<u8>,
    ) -> Result<Vec<u128>, Error> {
        // fn parameters
        f_array[1] = 0;
        d_array[1] = element;
        // declaration
        let mut function_f_array: [u8; 13] = vec![u8::default(); 13];
        function_f_array[1] = 0;
        let mut function_d_array: Vec<u128> = vec![u128::default(); 1];
        function_d_array[self.data().storage_f_array.len()] = element;
        // assign value
        self.data().storage_f_array[1] = 0;
        self.data().storage_d_array[self.data().storage_f_array.len()] = 0;
        self.data().storage_mapping.insert(&1, &(element));
        // assign array type
        function_d_array[1 + element] = self.data().storage_f_array[0];
        function_d_array[1 / element] = self.data().storage_d_array[1 + element];
        element = self
            .data()
            .storage_mapping
            .get(&element + 1)
            .unwrap_or_default();
        // nested array
        self.data().storage_d_struct_array[1]
            .test_struct
            .struct_f_array[1] = 0;
        self.data().storage_d_struct_array[1]
            .test_struct
            .struct_d_array[1] = 0;
        self.data().storage_d_struct_array[1]
            .test_struct
            .struct_mapping
            .get(&1)
            .unwrap_or_default() = 0;
        element = self.data().storage_d_struct_array[1]
            .test_struct
            .struct_f_array[1];
        element = self.data().storage_d_struct_array[1]
            .test_struct
            .struct_d_array[1];
        element = self.data().storage_d_struct_array[1]
            .test_struct
            .struct_mapping
            .get(&1)
            .unwrap_or_default();
        // struct fields
        let mut test_struct: TestStruct;
        test_struct.struct_f_array[1] = element;
        test_struct.struct_d_array[1] = d_array.len();
        test_struct.struct_mapping.insert(&1, &(element));
        // nested struct fields
        let mut nested_test_struct: NestedTestStruct;
        nested_test_struct.test_struct.struct_f_array[1] = element;
        nested_test_struct.test_struct.struct_d_array[1] = 0;
        nested_test_struct
            .test_struct
            .struct_mapping
            .insert(&1, &(element));
        // assign struct field
        function_d_array[1] = test_struct.struct_f_array[1];
        function_d_array[1] = nested_test_struct.test_struct.struct_d_array[1];
        element = test_struct.struct_mapping.get(&1).unwrap_or_default();
        // push
        function_d_array.push(1);
        self.data().storage_d_array.push(element);
        nested_test_struct.test_struct.struct_d_array[1].push(element);
        self.data().storage_d_struct_array[1]
            .test_struct
            .struct_d_array[1]
            .push(1);
        // pop
        function_d_array.pop();
        self.data().storage_d_array.pop();
        nested_test_struct.test_struct.struct_d_array[1].pop();
        self.data().storage_d_struct_array[1]
            .test_struct
            .struct_d_array[1]
            .pop();
        // delete
        self.data().storage_mapping.remove(&1);
        test_struct.struct_mapping.remove(&1);
        return Ok(function_d_array)
    }

}
