// MIT License

// Copyright (c) 2022 Supercolony

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::structures::Contract;

pub fn generate_mermaid(vec: Vec<Contract>) -> String {
    let mut out = String::new();

    out.push_str("graph TD");

    for contract in vec {
        out.push_str(format!("subgraph {}", contract.name.clone()).as_str());

        for storage_field in contract.fields {
            out.push_str(
                format!(
                    "{}[({})]:::storage",
                    storage_field.name.clone(),
                    storage_field.name.clone()
                )
                .as_str(),
            )
        }

        for function in contract.functions.clone() {
            out.push_str(
                format!(
                    "{}[{}]:::{}",
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
                            format!("{} --> {}", function.header.name.clone(), member).as_str(),
                        );
                    }
                    crate::structures::Call::ReadStorage(member) => {
                        out.push_str(
                            format!("{} -.-> {}", function.header.name.clone(), member).as_str(),
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
