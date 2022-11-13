use std::{
    fs,
    process::Command,
};

#[test]
fn erc20_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC20/ERC20/lib.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/contracts/ERC20/ERC20.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC20/ERC20/lib.rs").unwrap()
    );
}

#[test]
fn erc721_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC721/ERC721/lib.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/contracts/ERC721/ERC721.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC721/ERC721/lib.rs").unwrap()
    );
}

#[test]
fn erc1155_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC1155/ERC1155/lib.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/contracts/ERC1155/ERC1155.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC1155/ERC1155/lib.rs").unwrap()
    );
}

#[test]
fn access_control_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/AccessControl/AccessControl/lib.rs").unwrap();

    Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "examples/contracts/AccessControl/AccessControl.sol",
        ])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/AccessControl/AccessControl/lib.rs").unwrap()
    );
}

#[test]
fn solang_example_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/SolangExample/example/lib.rs").unwrap();

    Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "examples/contracts/SolangExample/example.sol",
        ])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/SolangExample/example/lib.rs").unwrap()
    );
}

#[test]
fn flipper_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/Flipper/flipper/lib.rs").unwrap();

    Command::new("cargo")
        .args(["+nightly", "run", "examples/contracts/Flipper/flipper.sol"])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/Flipper/flipper/lib.rs").unwrap()
    );
}

#[test]
fn primitives_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/Primitives/Primitives/lib.rs").unwrap();

    Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "examples/contracts/Primitives/Primitives.sol",
        ])
        .output()
        .expect("failed to execute process");

    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/Primitives/Primitives/lib.rs").unwrap()
    );
}

#[test]
fn ierc20_is_not_changed() {
    let file = fs::read_to_string("examples/interfaces/IERC20/IERC20/lib.rs").unwrap();

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
    let file = fs::read_to_string("examples/interfaces/IERC721/IERC721/lib.rs").unwrap();

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
    let file = fs::read_to_string("examples/interfaces/IERC1155/IERC1155/lib.rs").unwrap();

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
    let file =
        fs::read_to_string("examples/interfaces/IAccessControl/IAccessControl/lib.rs").unwrap();

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
