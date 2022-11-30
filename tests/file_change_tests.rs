use convert_case::{
    Case::Snake,
    Casing,
};
use std::{
    fs,
    process::Command,
};

macro_rules! implementation_file {
    ($origin:expr,$contract_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/contracts/{}/{}/{}/{}",
            $origin,
            $contract_name,
            $contract_name,
            $contract_name.to_case(Snake),
            $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! contract_file {
    ($origin:expr,$contract_name:expr,$file_name:expr) => {
        fs::read_to_string(format!(
            "{}/contracts/{}/{}/contract/{}",
            $origin, $contract_name, $contract_name, $file_name
        ))
        .unwrap_or_else(|err| panic!("{err:?}"))
    };
}

macro_rules! test_case {
    ($contract_name:expr) => {
        let contract_cargo = contract_file!("tests", $contract_name, "Cargo.toml");
        let contract_lib = contract_file!("tests", $contract_name, "lib.rs");

        let impl_cargo = implementation_file!("tests", $contract_name, "Cargo.toml");
        let impl_impl = implementation_file!("tests", $contract_name, "impls.rs");
        let impl_lib = implementation_file!("tests", $contract_name, "lib.rs");
        let impl_traits = implementation_file!("tests", $contract_name, "traits.rs");

        Command::new("cargo")
            .args([
                "+nightly",
                "run",
                format!(
                    "examples/contracts/{}/{}.sol",
                    $contract_name, $contract_name
                )
                .as_str(),
            ])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            contract_cargo,
            contract_file!("examples", $contract_name, "Cargo.toml")
        );
        assert_eq!(
            contract_lib,
            contract_file!("examples", $contract_name, "lib.rs")
        );

        assert_eq!(
            impl_cargo,
            implementation_file!("examples", $contract_name, "Cargo.toml")
        );
        assert_eq!(
            impl_impl,
            implementation_file!("examples", $contract_name, "impls.rs")
        );
        assert_eq!(
            impl_lib,
            implementation_file!("examples", $contract_name, "lib.rs")
        );
        assert_eq!(
            impl_traits,
            implementation_file!("examples", $contract_name, "traits.rs")
        );
    };
}

#[test]
fn erc20_is_not_changed() {
    test_case!("ERC20");
}

#[test]
fn erc721_is_not_changed() {
    test_case!("ERC721");
}

#[test]
fn erc1155_is_not_changed() {
    test_case!("ERC1155");
}

#[test]
fn access_control_is_not_changed() {
    test_case!("AccessControl");
}

#[test]
fn solang_example_is_not_changed() {
    test_case!("example");
}

#[test]
fn flipper_is_not_changed() {
    test_case!("flipper");
}

#[test]
fn primitives_is_not_changed() {
    test_case!("Primitives");
}

#[test]
fn ierc20_is_not_changed() {
    let file = fs::read_to_string("tests/interfaces/erc20.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/interfaces/IERC20/IERC20.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC20/IERC20/lib.rs").unwrap()
    );
}

#[test]
fn ierc721_is_not_changed() {
    let file = fs::read_to_string("tests/interfaces/erc721.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/interfaces/IERC721/IERC721.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC721/IERC721/lib.rs").unwrap()
    );
}

#[test]
fn ierc1155_is_not_changed() {
    let file = fs::read_to_string("tests/interfaces/erc1155.rs").unwrap();

    Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "examples/interfaces/IERC1155/IERC1155.sol",
        ])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC1155/IERC1155/lib.rs").unwrap()
    );
}

#[test]
fn iaccess_control_is_not_changed() {
    let file = fs::read_to_string("tests/interfaces/access_control.rs").unwrap();

    Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "examples/interfaces/IAccessControl/IAccessControl.sol",
        ])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IAccessControl/IAccessControl/lib.rs").unwrap()
    );
}
