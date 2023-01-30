use std::{
    fs,
    process::Command,
};

macro_rules! contract_file {
    ($origin:expr,$mod_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/generated/contracts/{}/{}",
            $origin, $mod_name, $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! test_file {
    ($origin:expr,$mod_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/generated/src/{}/{}",
            $origin, $mod_name, $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! test_case_contract {
    ($folder_name:expr,$mod_name:expr) => {
        let contract_cargo = contract_file!("tests", $mod_name, "Cargo.toml");
        let contract_lib = contract_file!("tests", $mod_name, "lib.rs");

        let impl_file = test_file!("tests", "impls", format!("{}.rs", $mod_name));
        let trait_file = test_file!("tests", "traits", format!("{}.rs", $mod_name));

        Command::new("cargo")
            .args([
                "+nightly",
                "run",
                format!("examples/contracts/{}.sol", $folder_name).as_str(),
            ])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            contract_cargo,
            contract_file!("examples", $mod_name, "Cargo.toml")
        );
        assert_eq!(
            contract_lib,
            contract_file!("examples", $mod_name, "lib.rs")
        );

        assert_eq!(
            impl_file,
            test_file!("examples", "impls", format!("{}.rs", $mod_name))
        );
        assert_eq!(
            trait_file,
            test_file!("examples", "traits", format!("{}.rs", $mod_name))
        );
    };
}

#[test]
fn erc20_is_not_changed() {
    test_case_contract!("ERC20", "erc_20");
}

#[test]
fn erc721_is_not_changed() {
    test_case_contract!("ERC721", "erc_721");
}

#[test]
fn erc1155_is_not_changed() {
    test_case_contract!("ERC1155", "erc_1155");
}

#[test]
fn access_control_is_not_changed() {
    test_case_contract!("AccessControl", "access_control");
}

#[test]
fn solang_example_is_not_changed() {
    test_case_contract!("example", "example");
}

#[test]
fn flipper_is_not_changed() {
    test_case_contract!("flipper", "flipper");
}

#[test]
fn primitives_is_not_changed() {
    test_case_contract!("Primitives", "primitives");
}

#[test]
fn array_contract_is_not_changed() {
    test_case_contract!("ArrayContract", "array_contract");
}

#[test]
fn comment_contract_is_not_changed() {
    test_case_contract!("CommentContract", "comment_contract");
}

#[test]
fn function_contract_is_not_changed() {
    test_case_contract!("FunctionContract", "function_contract");
}

#[test]
fn struct_contract_is_not_changed() {
    test_case_contract!("StructContract", "struct_contract");
}
