use crate::structures::Contract;

// Lore: Triton was the father of little mermaid. 
// Since Triton resembles Poseidon, the mermaid generator should be Poseidon

pub fn generate_mermaid(vec: Vec<Contract>) -> String {
    let mut out = String::new();

    out.push_str("graph TD");

    for contract in vec {
        out.push_str(format!("subgraph {}", contract.name.clone()).as_str());

        for storage_field in contract.fields {
            out.push_str(
                format!(
                    "s_{}_{}[({})]:::storage",
                    contract.name,
                    storage_field.name.clone(),
                    storage_field.name.clone()
                )
                .as_str(),
            )
        }

        for function in contract.functions.clone() {
            out.push_str(
                format!(
                    "f_{}_{}[{}]:::{}",
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
            for call in function.calls {
                match call {
                    crate::structures::Call::Read(member)
                    | crate::structures::Call::Write(member) => {
                        out.push_str(
                            format!(
                                "f_{}_{} --> {}",
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
                                "f_{}_{} -.-> {}",
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
    }

    out.push_str("classDef storage fill:#ff00ff,stroke:#333,stroke-width:2px;");
    out.push_str("classDef external fill:#ff0000,stroke:#333,stroke-width:2px;");
    out.push_str("classDef external_view fill:#ffff00,stroke:#333,stroke-width:2px;");
    out.push_str("classDef actor fill:#00ff00,stroke:#333,stroke-width:2px;");
    out.push_str(
        "classDef internal fill:#ff0000,stroke:#333,stroke-width:2px,stroke-dasharray: 5 5;",
    );
    out.push_str(
        "classDef internal_view fill:#ffff00,stroke:#333,stroke-width:2px,stroke-dasharray: 5 5;",
    );

    out
}
