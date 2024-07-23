use std::collections::HashMap;

use crate::structures::{
    Call,
    Contract,
};

// Lore: Triton was the father of little mermaid.
// Since Triton resembles Poseidon, the mermaid generator should be Poseidon

pub fn generate_mermaid(
    vec: &Vec<Contract>,
    slots_map: &HashMap<String, Vec<String>>,
    omit_read_storage: bool,
) -> String {
    let mut out = String::new();

    out.push_str("graph LR\n");

    let mut write_access = HashMap::new();

    for contract in vec.clone() {
        for function in contract.functions.clone() {
            for call in function.calls {
                if let Call::Library(..) = call {
                    // @todo this must be processed before
                    continue
                }
                if function.header.view {
                    continue;
                }
                write_access.insert(
                    format!("f_{}_{}", contract.name, function.header.name.clone(),),
                    (),
                );
                if omit_read_storage && call.is_read_storage() {
                    continue
                }
                write_access.insert(call.to_string(), ());
            }
        }
    }

    for contract in vec {
        let mut sub_graph = String::new();
        let mut contains_stuff = false;

        sub_graph.push_str(format!("subgraph {}\n", contract.name.clone()).as_str());

        sub_graph.push('\n');

        if !contract.fields.is_empty() {
            sub_graph.push_str("subgraph Storage\n");

            for storage_field in contract.fields.clone() {
                if !write_access.contains_key(
                    format!("s_{}_{}", contract.name, storage_field.name.clone()).as_str(),
                ) {
                    continue
                }
                contains_stuff = true;
                sub_graph.push_str(
                    format!(
                        "s_{}_{}[({})]:::storage\n",
                        contract.name,
                        storage_field.name.clone(),
                        storage_field.name.clone()
                    )
                    .as_str(),
                )
            }

            sub_graph.push_str("end\n");
            sub_graph.push('\n');
        }

        for function in contract.functions.clone() {
            if (function.header.view
                && !write_access.contains_key(
                    format!("f_{}_{}", contract.name, function.header.name.clone()).as_str(),
                ))
                || !function.header.external
            {
                continue
            }
            contains_stuff = true;
            sub_graph.push_str(
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
            contains_stuff = true;
            sub_graph.push_str(
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

        sub_graph.push('\n');
        sub_graph.push_str("end\n");

        for function in contract.functions.clone() {
            if !write_access.contains_key(
                format!("f_{}_{}", contract.name, function.header.name.clone()).as_str(),
            ) {
                continue
            }
            contains_stuff = true;
            // one function may call a member multiple times, we do not care
            let mut filtered_calls = function.calls.clone();
            filtered_calls.sort();
            filtered_calls.dedup();
            filtered_calls = filtered_calls
                .iter()
                .filter(|call| {
                    if let Call::ReadStorage(call_type, contract, member) = call {
                        !filtered_calls.contains(&Call::Write(
                            call_type.clone(),
                            contract.clone(),
                            member.clone(),
                        )) && filtered_calls.contains(&Call::WriteStorage(
                            call_type.clone(),
                            contract.clone(),
                            member.clone(),
                        ))
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            // one function may also read and write to storage, we will favor write

            for call in filtered_calls {
                match call {
                    crate::structures::Call::Read(..)
                    | crate::structures::Call::Write(..)
                    | crate::structures::Call::WriteStorage(..) => {
                        sub_graph.push_str(
                            format!(
                                "f_{}_{} --> {}\n",
                                contract.name,
                                function.header.name.clone(),
                                call.to_string()
                            )
                            .as_str(),
                        );
                    }
                    crate::structures::Call::ReadStorage(..) => {
                        if !omit_read_storage {
                            sub_graph.push_str(
                                format!(
                                    "f_{}_{} -.-> {}\n",
                                    contract.name,
                                    function.header.name.clone(),
                                    call.to_string()
                                )
                                .as_str(),
                            );
                        }
                    }
                    _ => (),
                }
            }
        }

        sub_graph.push('\n');

        if contains_stuff {
            out.push_str(&sub_graph);
        }
    }

    for slot in slots_map {
        for field in slot.1 {
            if !write_access.contains_key(format!("s_{}_{}", slot.0, field).as_str()) {
                continue
            }
            out.push_str(format!("s_{}_{field}[({field})]:::storage\n", slot.0).as_str());
        }
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
