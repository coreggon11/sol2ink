use std::collections::HashMap;

use crate::structures::{
    Call,
    Contract,
};

// Lore: Triton was the father of little mermaid.
// Since Triton resembles Poseidon, the mermaid generator should be Poseidon

pub fn generate_mermaid(vec: Vec<Contract>) -> String {
    let mut out = String::new();

    out.push_str("graph TD\n");

    let mut write_access = HashMap::new();

    for contract in vec.clone() {
        for function in contract.functions.clone() {
            for call in function.calls {
                if function.header.view {
                    continue;
                }
                write_access.insert(
                    format!("f_{}_{}", contract.name, function.header.name.clone(),),
                    (),
                );
                match call {
                    crate::structures::Call::Read(member)
                    | crate::structures::Call::Write(member)
                    | crate::structures::Call::ReadStorage(member) => {
                        write_access.insert(member, ());
                    }
                }
            }
        }
    }

    for contract in vec {
        out.push_str(format!("subgraph {}\n", contract.name.clone()).as_str());

        out.push('\n');

        out.push_str("subgraph Storage\n");

        for storage_field in contract.fields {
            if !write_access.contains_key(
                format!("s_{}_{}", contract.name, storage_field.name.clone()).as_str(),
            ) {
                continue
            }
            out.push_str(
                format!(
                    "s_{}_{}[({})]:::storage\n",
                    contract.name,
                    storage_field.name.clone(),
                    storage_field.name.clone()
                )
                .as_str(),
            )
        }

        out.push_str("end\n");
        out.push('\n');

        for function in contract.functions.clone() {
            if !write_access.contains_key(
                format!("f_{}_{}", contract.name, function.header.name.clone()).as_str(),
            ) || !function.header.external
            {
                continue
            }
            out.push_str(
                format!(
                    "f_{}_{}[{}]:::{}\n",
                    contract.name,
                    function.header.name.clone(),
                    function.header.name.clone(),
                    match (function.header.external, function.header.view) {
                        (true, true) => "external_view",
                        (true, false) => "external",
                        (false, true) => "internal_view",
                        (false, false) => "internal",
                    }
                )
                .as_str(),
            )
        }

        for function in contract.functions.clone() {
            if !write_access.contains_key(
                format!("f_{}_{}", contract.name, function.header.name.clone()).as_str(),
            ) || function.header.external
            {
                continue
            }
            out.push_str(
                format!(
                    "f_{}_{}[{}]:::{}\n",
                    contract.name,
                    function.header.name.clone(),
                    function.header.name.clone(),
                    match (function.header.external, function.header.view) {
                        (true, true) => "external_view",
                        (true, false) => "external",
                        (false, true) => "internal_view",
                        (false, false) => "internal",
                    }
                )
                .as_str(),
            )
        }

        out.push('\n');
        out.push_str("end\n");

        for function in contract.functions.clone() {
            if !write_access.contains_key(
                format!("f_{}_{}", contract.name, function.header.name.clone()).as_str(),
            ) {
                continue
            }
            // one function may call a member multiple times, we do not care
            let mut filtered_calls = function.calls.clone();
            filtered_calls.sort();
            filtered_calls.dedup();
            filtered_calls = filtered_calls
                .iter()
                .filter(|call| {
                    if let Call::ReadStorage(member) = call {
                        !filtered_calls.contains(&Call::Write(member.clone()))
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            // one function may also read and write to storage, we will favor write

            for call in filtered_calls {
                match call {
                    crate::structures::Call::Read(member)
                    | crate::structures::Call::Write(member) => {
                        out.push_str(
                            format!(
                                "f_{}_{} --> {}\n",
                                contract.name,
                                function.header.name.clone(),
                                member
                            )
                            .as_str(),
                        );
                    }
                    crate::structures::Call::ReadStorage(member) => {
                        out.push_str(
                            format!(
                                "f_{}_{} -.-> {}\n",
                                contract.name,
                                function.header.name.clone(),
                                member
                            )
                            .as_str(),
                        );
                    }
                }
            }
        }

        out.push('\n');
    }

    out.push_str("classDef storage fill:#ff00ff,stroke:#333,stroke-width:2px;\n");
    out.push_str("classDef external fill:#ff0000,stroke:#333,stroke-width:2px;\n");
    out.push_str("classDef external_view fill:#ffff00,stroke:#333,stroke-width:2px;\n");
    out.push_str("classDef actor fill:#00ff00,stroke:#333,stroke-width:2px;\n");
    out.push_str(
        "classDef internal fill:#ff0000,stroke:#333,stroke-width:2px,stroke-dasharray: 5 5;\n",
    );
    out.push_str(
        "classDef internal_view fill:#ffff00,stroke:#333,stroke-width:2px,stroke-dasharray: 5 5;\n",
    );

    out
}
