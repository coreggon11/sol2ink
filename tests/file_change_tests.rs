use std::{
    fs,
    process::Command,
};

macro_rules! test_case_file {
    ($dest:expr,$file_name:expr) => {
        let file = fs::read_to_string(format!("tests/{}/{}.rs", $dest, $file_name)).unwrap();

        Command::new("cargo")
            .args([
                "+nightly",
                "run",
                format!("examples/{}/{}/{}.sol", $dest, $file_name, $file_name).as_str(),
            ])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            file,
            fs::read_to_string(format!(
                "examples/{}/{}/{}.rs",
                $dest, $file_name, $file_name
            ))
            .unwrap()
        );
    };
}

macro_rules! contract_file {
    ($origin:expr,$folder_name:expr,$mod_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/contracts/{}/{}/{}",
            $origin, $folder_name, $mod_name, $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! test_case_contract {
    ($folder_name:expr,$mod_name:expr) => {
        let contract_cargo = contract_file!("tests", $folder_name, "contract", "Cargo.toml");
        let contract_lib = contract_file!("tests", $folder_name, "contract", "lib.rs");

        let impl_cargo = contract_file!("tests", $folder_name, $mod_name, "Cargo.toml");
        let impl_impl = contract_file!("tests", $folder_name, $mod_name, "impls.rs");
        let impl_lib = contract_file!("tests", $folder_name, $mod_name, "lib.rs");
        let impl_traits = contract_file!("tests", $folder_name, $mod_name, "traits.rs");

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
            contract_file!("examples", $folder_name, "contract", "Cargo.toml")
        );
        assert_eq!(
            contract_lib,
            contract_file!("examples", $folder_name, "contract", "lib.rs")
        );

        assert_eq!(
            impl_cargo,
            contract_file!("examples", $folder_name, $mod_name, "Cargo.toml")
        );
        assert_eq!(
            impl_impl,
            contract_file!("examples", $folder_name, $mod_name, "impls.rs")
        );
        assert_eq!(
            impl_lib,
            contract_file!("examples", $folder_name, $mod_name, "lib.rs")
        );
        assert_eq!(
            impl_traits,
            contract_file!("examples", $folder_name, $mod_name, "traits.rs")
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
fn ierc20_is_not_changed() {
    test_case_file!("interfaces", "IERC20");
}

#[test]
fn ierc721_is_not_changed() {
    test_case_file!("interfaces", "IERC721");
}

#[test]
fn ierc1155_is_not_changed() {
    test_case_file!("interfaces", "IERC1155");
}

#[test]
fn iaccess_control_is_not_changed() {
    test_case_file!("interfaces", "IAccessControl");
}

#[test]
fn safe_math_is_not_changed() {
    test_case_file!("libraries", "SafeMath");
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
