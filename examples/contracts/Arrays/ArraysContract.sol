// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.7.0) (token/ERC1155/ERC1155.sol)

pragma solidity ^0.8.0;

contract Arrays {
    struct TestStruct {
        mapping(uint256 => uint256) struct_mapping;
        bytes32 struct_f_array;
        uint256[] struct_d_array;
    }

    struct NestedTestStruct {
        TestStruct test_struct;
        uint8 uint_field;
    }

    mapping(uint256 => uint256) storage_mapping;
    bytes32 storage_f_array;
    uint256[] storage_d_array;
    NestedTestStruct[] storage_d_struct_array;

    function work_with_arrays(uint256 element, bytes8 f_array, uint8[] d_array) private pure returns (uint256[] memory) {
        // fn parameters
        f_array[1] = 0;
        d_array[1] = element;

        // declaration (error with f_array)
        uint256[] memory function_d_array = new uint256[](1);
        function_d_array[storage_f_array.length] = element;

        // assign value
        storage_f_array[1] = 0;
        storage_d_array[storage_f_array.length] = 0;
        storage_mapping[1] = element;

        // assign array type
        function_d_array[1+element] = storage_f_array[0];
        function_d_array[1/element] = storage_d_array[1+element];
        element = storage_mapping[element+1];

        // nested array
        storage_d_struct_array[1].test_struct.struct_f_array[1] = 0;
        storage_d_struct_array[1].test_struct.struct_d_array[1] = 0;
        storage_d_struct_array[1].test_struct.struct_mapping[1] = 0;
        element = storage_d_struct_array[1].test_struct.struct_f_array[1];
        element = storage_d_struct_array[1].test_struct.struct_d_array[1];
        element = storage_d_struct_array[1].test_struct.struct_mapping[1];

        // struct fields
        TestStruct memory test_struct;
        test_struct.struct_f_array[1] = element;
        test_struct.struct_d_array[1] = d_array.length;
        test_struct.struct_mapping[1] = element;

        // nested struct fields
        NestedTestStruct memory nested_test_struct;
        nested_test_struct.test_struct.struct_f_array[1] = element;
        nested_test_struct.test_struct.struct_d_array[1] = 0;
        nested_test_struct.test_struct.struct_mapping[1] = element;

        // assign struct field
        function_d_array[1] = test_struct.struct_f_array[1];
        function_d_array[1] = nested_test_struct.test_struct.struct_d_array[1];
        element = test_struct.struct_mapping[1];

        // push
        function_d_array.push(1);
        storage_d_array.push(element);
        nested_test_struct.test_struct.struct_d_array[1].push(element);
        storage_d_struct_array[1].test_struct.struct_d_array[1].push(1);

        // pop
        function_d_array.pop();
        storage_d_array.pop();
        nested_test_struct.test_struct.struct_d_array[1].pop();
        storage_d_struct_array[1].test_struct.struct_d_array[1].pop();

        return function_d_array;
    }
}