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

use crate::structures::*;
use convert_case::{
    Case::{
        self,
        Pascal,
        Snake,
        UpperSnake,
    },
    Casing,
};
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::*;
use std::{
    collections::HashMap,
    str::FromStr,
};

// constant vector of rust keywords which are not keywords in solidity
const RUST_KEYWORDS: [&str; 27] = [
    "const", "crate", "extern", "fn", "impl", "in", "loop", "mod", "move", "mut", "pub", "ref",
    "self", "Self", "trait", "unsafe", "use", "where", "become", "box", "final", "priv", "unsized",
    "async", "await", "dyn", "union",
];

/// Assembles the TokenStream of an ink! contract from the parsed contract struct
pub fn assemble_contract(contract: &Contract) -> TokenStream {
    let mod_name = format_ident!("{}", contract.name.to_case(Snake));
    let contract_name = format_ident!("{}Contract", contract.name);
    let trait_name = format_ident!("{}", contract.name);
    let signature = signature();
    let events = assemble_events(&contract.events);
    let storage = assemble_storage(&contract.name);
    let constructor = assemble_constructor(&contract.constructor, &contract.fields);
    let constants = assemble_constants(&contract.fields);
    let emit_functions = assemble_contract_emit_functions(&contract.events);
    let base = contract
        .base
        .iter()
        .map(|base| TokenStream::from_str(base).unwrap())
        .collect::<Vec<_>>();
    let internal = if emit_functions.is_empty() {
        quote!()
    } else {
        quote!(
            impl generated::impls:: #mod_name ::Internal for #contract_name {
                _blank_!();
                #emit_functions
            }
        )
    };

    let contract = quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        #signature
        #[openbrush::contract]
        pub mod #mod_name {
            use openbrush::traits::Storage;
            use generated::*;
            use ink::lang::codegen::Env;
            use ink::lang::codegen::EmitEvent;
            _blank_!();
            #constants
            #events
            #storage
            _blank_!();
            impl #trait_name for #contract_name {}
            #internal
            #(_blank_!(); impl #base for #contract_name {})*

            _blank_!();
            impl #contract_name {
                #constructor
            }
        }
    };

    contract
}

/// Assembles the TokenStream of an ink! contract implementation file from the parsed contract struct
pub fn assemble_impl(contract: &Contract) -> TokenStream {
    let trait_name = format_ident!("{}", contract.name);
    let signature = signature();
    let imports = Vec::from_iter(&contract.imports);
    let data = assemble_data_struct(&contract.fields);
    let getters = assemble_getters(&contract.fields);
    let mut modifiers_map = HashMap::new();
    contract.modifiers.iter().for_each(|function| {
        modifiers_map.insert(function.header.name.clone(), function.clone());
    });

    let functions = assemble_functions(
        &contract
            .functions
            .iter()
            .filter(|f| f.header.external)
            .cloned()
            .collect::<Vec<_>>(),
        false,
    );
    let internal_trait = assemble_function_headers(
        &contract
            .functions
            .iter()
            .filter(|f| !f.header.external)
            .map(|f| f.clone().header)
            .collect::<Vec<_>>(),
    );
    let internal_functions = assemble_functions(
        &contract
            .functions
            .iter()
            .filter(|f| !f.header.external)
            .cloned()
            .collect::<Vec<_>>(),
        false,
    );
    let (emit_function_headers, impl_emit_functions) = assemble_emit_functions(&contract.events);
    let modifiers = assemble_modifiers(&contract.modifiers, &trait_name);

    let contract = quote! {
        #signature
        pub use crate::{
            impls,
            traits::*,
        };
        #(#imports)*
        use openbrush::traits::Storage;
        _blank_!();
        #data
        _blank_!();
        #modifiers
        _blank_!();
        impl <T: Storage<Data>> #trait_name for T {
            #functions
            #getters
        }
        _blank_!();
        pub trait Internal {
            #internal_trait
            #emit_function_headers
        }
        _blank_!();
        impl<T: Storage<Data>> Internal for T {
            #internal_functions
            #impl_emit_functions
        }
    };

    contract
}

/// Assembles the TokenStream of an ink! trait from the parsed contract struct
pub fn assemble_trait(contract: &Contract) -> TokenStream {
    let trait_name = TokenStream::from_str(&contract.name).unwrap();
    let ref_name = TokenStream::from_str(&format!("{}Ref", contract.name)).unwrap();
    let signature = signature();
    let imports = Vec::from_iter(&contract.imports);
    let enums = assemble_enums(&contract.enums);
    let structs = assemble_structs(&contract.structs);
    let getters_trait = assemble_getters_trait(&contract.fields, &contract.functions);
    let function_headers = assemble_function_headers(
        &contract
            .functions
            .clone()
            .iter()
            .filter(|f| f.header.external)
            .map(|f| f.header.clone())
            .collect::<Vec<_>>(),
    );

    quote! {
        #signature
        #(#imports)*
        use scale::{
            Decode,
            Encode,
        };
        _blank_!();
        #[derive(Debug, Encode, Decode, PartialEq, Eq)]
        #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
        pub enum Error {
            Custom(String),
        }
        _blank_!();
        #enums
        _blank_!();
        #structs
        _blank_!();
        #[openbrush::wrapper]
        pub type #ref_name = dyn #trait_name;
        _blank_!();
        #[openbrush::trait_definition]
        pub trait #trait_name {
            #function_headers
            #getters_trait
        }
    }
}

/// Assembles the TokenStream of a lib file of the ink! project
pub fn assemble_lib() -> TokenStream {
    quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        pub mod impls;
        pub mod traits;
        pub mod libs;

        pub use impls::*;
        pub use traits::*;
        pub use libs::*;
    }
}

/// Assembles the TokenStream of a mod file of a folder, containing all modules provided
pub fn assemble_mod(mods: &[String]) -> TokenStream {
    let tokens = mods
        .iter()
        .map(|name| TokenStream::from_str(name).unwrap())
        .collect::<Vec<_>>();

    quote! {
        #(pub mod #tokens; pub use #tokens::*; _blank_!(); )*
    }
}

/// Assembles the TokenStream of an ink! trait from the parsed interface struct
pub fn assemble_interface(interface: Interface) -> TokenStream {
    let interface_name = TokenStream::from_str(&interface.name).unwrap();
    let interface_name_ref = TokenStream::from_str(&format!("{}Ref", interface.name)).unwrap();
    let signature = signature();
    let imports = Vec::from_iter(&interface.imports);
    let events = assemble_events(&interface.events);
    let enums = assemble_enums(&interface.enums);
    let structs = assemble_structs(&interface.structs);
    let function_headers = assemble_function_headers(&interface.function_headers);

    let interface = quote! {
        #signature
        #(#imports)*
        _blank_!();
        #events
        #enums
        #structs
        #[openbrush::wrapper]
        pub type #interface_name_ref = dyn #interface_name;
        _blank_!();
        #[openbrush::trait_definition]
        pub trait #interface_name {
            #function_headers
        }
    };

    interface
}

/// Assembles the TokenStream of a library from the parsed library struct
pub fn assemble_library(library: Library) -> TokenStream {
    let signature = signature();
    let imports = Vec::from_iter(&library.imports);
    let events = assemble_events(&library.events);
    let enums = assemble_enums(&library.enums);
    let structs = assemble_structs(&library.structs);
    let constants = assemble_constants(&library.fields);
    let functions = assemble_functions(&library.functions, true);

    let library = quote! {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![feature(min_specialization)]
        _blank_!();
        #(#imports)*
        _blank_!();
        #signature
        _blank_!();
        pub enum Error {
            Custom(String),
        }
        _blank_!();

        #constants
        #events
        #enums
        #structs
        #functions
    };

    library
}

/// Assembles the TokenStream of Enums from the parsed Enum structs
fn assemble_enums(enums: &[Enum]) -> TokenStream {
    let mut output = TokenStream::new();

    for enumeration in enums.iter() {
        let enum_name =
            TokenStream::from_str(&format_expression(&enumeration.name, Pascal)).unwrap();
        let mut values = TokenStream::new();

        // assemble enum values
        for value in enumeration.values.iter() {
            let value_name = TokenStream::from_str(&value.name.to_case(Pascal)).unwrap();

            values.extend(quote! {
                #value_name,
            });
        }

        output.extend(quote! {
            pub enum #enum_name {
                #values
            }
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of ink! events from the parsed Event structs
fn assemble_events(events: &[Event]) -> TokenStream {
    let mut output = TokenStream::new();

    for event in events.iter() {
        let event_name = TokenStream::from_str(&event.name).unwrap();
        let mut event_fields = TokenStream::new();

        // assemble event fields
        for event_field in event.fields.iter() {
            if event_field.indexed {
                event_fields.extend(quote! {
                    #[ink(topic)]
                });
            }

            let event_field_name = format_ident!("{}", format_expression(&event_field.name, Snake));
            let event_field_type = &event_field.field_type;

            event_fields.extend(quote! {
                #event_field_name: #event_field_type,
            });
        }

        output.extend(quote! {
            #[ink(event)]
            pub struct #event_name
            {
                #event_fields
            }
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of contract's storage fields from the parsed ContractField structs
fn assemble_data_struct(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();
    let mut storage_fields = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| !field.constant) {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = &field.field_type;

        storage_fields.extend(quote! {
            pub #field_name: #field_type,
        });
    }

    output.extend(quote! {
        pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
        _blank_!();
        #[derive(Default, Debug)]
        #[openbrush::upgradeable_storage(STORAGE_KEY)]
        pub struct Data {
            #storage_fields
            pub _reserved: Option<()>,
        }
    });

    output
}

/// Assembles the TokenStream of getters for public fields of the contract storage from the parsed ContractField structs
fn assemble_getters(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields
        .iter()
        .filter(|field| !field.constant && field.public)
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = &field.field_type;

        output.extend(quote! {
            fn #field_name(&self) -> #field_type {
                self.data().#field_name
            }
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of getter function descriptions for public fields of the contract storage
/// from the parsed ContractField structs
fn assemble_getters_trait(fields: &[ContractField], functions: &[Function]) -> TokenStream {
    let mut output = TokenStream::new();

    let mut function_mapping = HashMap::new();
    for function in functions.iter().map(|function| &function.header.name) {
        function_mapping.insert(function, ());
    }

    // assemble storage fields
    for field in fields
        .iter()
        .filter(|field| !field.constant)
        .filter(|field| function_mapping.contains_key(&field.name))
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let field_type = &field.field_type;

        output.extend(quote! {
            #[ink(message)]
            fn #field_name(&self) -> #field_type;
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of ink! contract
fn assemble_storage(contract_name: &String) -> TokenStream {
    let mut output = TokenStream::new();
    let contract_name = format_ident!("{}Contract", contract_name);

    output.extend(quote! {
        #[ink(storage)]
        #[derive(Default, Storage)]
        pub struct #contract_name {
            #[storage_field]
            data: impls::Data,
        }
    });

    output
}

/// Assembles the TokenStream of constant fields of the contract
fn assemble_constants(fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();

    // assemble storage fields
    for field in fields.iter().filter(|field| field.constant) {
        let field_name = format_ident!("{}", format_expression(&field.name, UpperSnake));
        let field_type = &field.field_type;
        let initial_value = field.initial_value.clone().unwrap();

        output.extend(quote! {
            pub const #field_name: #field_type = #initial_value;
        });
    }

    output.extend(quote! {
        _blank_!();
    });

    output
}

/// Assembles the TokenStream of structs from the parsed Struct structs
fn assemble_structs(structs: &[Struct]) -> TokenStream {
    let mut output = TokenStream::new();

    for structure in structs.iter() {
        let struct_name =
            TokenStream::from_str(&format_expression(&structure.name, Pascal)).unwrap();
        let mut struct_fields = TokenStream::new();

        // assemble struct fields
        for struct_field in structure.fields.iter() {
            let struct_field_name =
                format_ident!("{}", &format_expression(&struct_field.name, Snake));

            let struct_field_type = &struct_field.field_type;

            struct_fields.extend(quote! {
                #struct_field_name: #struct_field_type,
            });
        }

        output.extend(quote! {
            #[derive(Default, Encode, Decode)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
            pub struct #struct_name {
                #struct_fields
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of cosntructor from the parsed Function struct
/// If there are any fields with a preset value in the original contract, we will initialize them in the constructor
fn assemble_constructor(constructor: &Function, fields: &[ContractField]) -> TokenStream {
    let mut output = TokenStream::new();
    let mut params = TokenStream::new();
    let constructor_functions = &constructor.body;

    // assemble params
    for param in constructor.header.params.iter() {
        let param_name = format_ident!("{}", param.name.to_case(Snake));
        let param_type = &param.param_type;

        params.extend(quote! {
            #param_name: #param_type,
        });
    }

    let mut body = TokenStream::new();

    // assemble body
    body.extend(quote! {
        #constructor_functions
    });

    for field in fields
        .iter()
        .filter(|field| field.initial_value.is_some() && !field.constant)
    {
        let field_name = format_ident!("{}", field.name.to_case(Snake));
        let intial_value = field.initial_value.clone();

        body.extend(quote! {
            self.#field_name = #intial_value;
        });
    }

    output.extend(quote! {
        #[ink(constructor)]
        pub fn new(#params) -> Self{
            let mut instance = Self::default();
            #body
            instance
        }
        _blank_!();
    });

    output
}

/// Assembles the TokenStream of functions from the parsed Function structs
/// If we are creating functions for a library, we will not manipulate with contract's storage
fn assemble_functions(functions: &[Function], is_library: bool) -> TokenStream {
    let mut output = TokenStream::new();

    for function in functions.iter() {
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();
        let mut body = TokenStream::new();
        let mut function_modifiers = TokenStream::new();
        let invalid_modifiers = &function.invalid_modifiers;
        let invalid_modifiers_vec = &function.header.invalid_modifiers;
        let statement = function.body.clone();

        for function_modifier in function.header.modifiers.iter() {
            function_modifiers.extend(quote! {
                #function_modifier
            });
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!(
                "{}fn {}{}",
                if is_library {
                    String::from("pub ")
                } else if !function.header.external {
                    String::from("default ")
                } else {
                    String::new()
                },
                if !function.header.external && !is_library {
                    String::from("_")
                } else {
                    String::new()
                },
                format_expression(&function.header.name, Snake)
            ))
            .unwrap(),
        );

        // assemble view
        view.extend(match function.header.view {
            true => quote!(&self),
            false => quote!(&mut self),
        });

        // assemble params
        for param in function.header.params.iter() {
            let param_name = format_ident!("{}", &format_expression(&param.name, Snake));
            let param_type = &param.param_type;

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if !function.header.return_params.is_empty() {
            let mut params = TokenStream::new();

            for i in 0..function.header.return_params.len() {
                let param = &function.header.return_params[i];
                let param_type = &param.param_type;

                if i > 0 {
                    params.extend(quote! {,});
                }

                params.extend(quote! {
                    #param_type
                });

                if param.name != "_" {
                    let param_name =
                        TokenStream::from_str(&format_expression(&param.name, Snake)).unwrap();
                    body.extend(quote! {
                        let mut #param_name = Default::default();
                    })
                }
            }

            if function.header.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        // body
        body.extend(quote! {
            #statement
        });

        if function.header.return_params.is_empty() {
            body.extend(quote! {
                Ok(())
            });
        } else if function.header.return_params[0].name != "_" {
            let out = TokenStream::from_str(
                &function
                    .header
                    .return_params
                    .iter()
                    .map(|param| format_expression(&param.name, Snake))
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .unwrap();
            if !has_return_statement(&statement) {
                body.extend(
                    if function.header.return_params.len() > 1 {
                        quote! {
                            Ok((#out))
                        }
                    } else {
                        quote! {
                            Ok(#out)
                        }
                    },
                );
            }
        }

        let mut forgot_modifiers = TokenStream::new();
        for modifier in invalid_modifiers_vec {
            let (modifier_name, arguments) = match modifier {
                Expression::InvalidModifier(name, expressions) => (name, expressions),
                _ => unreachable!("Only invalid modifiers allowed here"),
            };
            // arguments map to modifier params
            if let Some(function) =
                invalid_modifiers.get(&(function.header.name.clone(), modifier_name.clone()))
            {
                if let Some(statement) = &function.body {
                    let vars = &function.header.params;
                    forgot_modifiers.extend(quote!(
                        #(let #vars = #arguments;)*
                    ));
                    match statement {
                        Statement::Block(statements) | Statement::UncheckedBlock(statements) => {
                            for statement in statements {
                                match statement.clone() {
                                    Statement::Expression(Expression::ModifierBody) => {}
                                    _ => forgot_modifiers.extend(quote!(#statement)),
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        output.extend(quote! {
            #function_modifiers
            #function_name(#view #params) -> Result<#return_params, Error> {
                #forgot_modifiers
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Helper function which returns true if the given statement contains a return statement
fn has_return_statement(statement: &Option<Statement>) -> bool {
    match statement {
        Some(statement) => {
            match statement {
                Statement::Block(statements) | Statement::UncheckedBlock(statements) => {
                    if statements.is_empty() {
                        return false
                    }
                    matches!(statements.last().unwrap(), Statement::Return(..))
                }
                _ => false,
            }
        }
        None => false,
    }
}

/// Assembles the TokenStream of internal emit functions of the contracts, which emit events
/// Returns the default implementation of such functions with empty body, which is then added to the implementation of the Internal trait
/// as well as the function definitions, which is used in the Internal trait definition
/// The TokenStream is generated based on the parsed events
fn assemble_emit_functions(events: &[Event]) -> (TokenStream, TokenStream) {
    let mut default_output = TokenStream::new();
    let mut impl_output = TokenStream::new();

    for event in events.iter() {
        let event_name =
            TokenStream::from_str(&format!("_emit_{}", &event.name.to_case(Snake))).unwrap();
        let mut event_args = TokenStream::new();
        let mut unnamed_event_args = TokenStream::new();

        // assemble event fields
        for event_field in event.fields.iter() {
            let event_field_name = format_ident!("{}", format_expression(&event_field.name, Snake));
            let event_field_type = &event_field.field_type;

            event_args.extend(quote! {
                #event_field_name: #event_field_type,
            });

            unnamed_event_args.extend(quote! {
                _: #event_field_type,
            });
        }

        default_output.extend(quote! {
            fn #event_name (&self, #event_args );
            _blank_!();
        });
        impl_output.extend(quote! {
            default fn #event_name (&self, #unnamed_event_args ) {}
            _blank_!();
        });
    }

    (default_output, impl_output)
}

/// Assembles the TokenStream of contract's emit functions, which then emit events
/// The TokenStream is generated based on the parsed events
fn assemble_contract_emit_functions(events: &[Event]) -> TokenStream {
    let mut output = TokenStream::new();

    for event in events.iter() {
        let fn_name =
            TokenStream::from_str(&format!("_emit_{}", &event.name.to_case(Snake))).unwrap();
        let mut event_args = TokenStream::new();
        let mut event_params = TokenStream::new();
        let event_name = TokenStream::from_str(&event.name).unwrap();

        // assemble event fields
        for event_field in event.fields.iter() {
            let event_field_name = format_ident!("{}", format_expression(&event_field.name, Snake));
            let event_field_type = &event_field.field_type;

            event_params.extend(quote! {
                #event_field_name: #event_field_type,
            });
            event_args.extend(quote! {
                #event_field_name,
            });
        }

        output.extend(quote! {
            fn #fn_name (&self, #event_params ) {
                self.env().emit_event(#event_name { #event_args });
            }
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of conract's modifiers from the parsed Function structs
fn assemble_modifiers(modifiers: &[Function], contract_name: &Ident) -> TokenStream {
    let mut output = TokenStream::new();

    for modifier in modifiers.iter() {
        let modifier_name = format_ident!("{}", format_expression(&modifier.header.name, Snake));
        let mut params = TokenStream::new();

        // assemble comments
        let body = &modifier.body;

        // assemble params
        for param in modifier.header.params.iter() {
            let param_name = format_ident!("{}", param.name.to_case(Snake));
            let param_type = &param.param_type;

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        output.extend(quote! {
            #[modifier_definition]
            pub fn #modifier_name<T, F, R>(instance: &mut T, body: F #params) -> Result<R, Error>
            where
                T: #contract_name,
                F: FnOnce(&mut T) -> Result<R, Error>
            {
                #body
            }
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of trait function headers from the parsed FunctionHeader structs
fn assemble_function_headers(function_headers: &[FunctionHeader]) -> TokenStream {
    let mut output = TokenStream::new();

    for header in function_headers.iter() {
        let mut message = TokenStream::new();
        let mut function_name = TokenStream::new();
        let mut view = TokenStream::new();
        let mut params = TokenStream::new();
        let mut return_params = TokenStream::new();

        // assemble message
        if header.external {
            if header.payable {
                message.extend(quote! {
                    #[ink(message, payable)]
                });
            } else {
                message.extend(quote! {
                    #[ink(message)]
                });
            }
        }

        // assemble function name
        function_name.extend(
            TokenStream::from_str(&format!(
                "fn {}{}",
                if header.external { "" } else { "_" },
                format_expression(&header.name, Snake)
            ))
            .unwrap(),
        );

        // assemble view
        view.extend(match header.view {
            true => quote!(&self),
            false => quote!(&mut self),
        });

        // assemble params
        for param in header.params.iter() {
            let param_name = format_ident!("{}", format_expression(&param.name, Snake));
            let param_type = &param.param_type;

            params.extend(quote! {
                , #param_name: #param_type
            });
        }

        // assemble return params
        if !header.return_params.is_empty() {
            let mut params = TokenStream::new();
            for i in 0..header.return_params.len() {
                let param_type = &header.return_params[i].param_type;

                if i > 0 {
                    params.extend(quote! {,});
                }
                params.extend(quote! {
                    #param_type
                });
            }

            if header.return_params.len() > 1 {
                return_params.extend(quote! {
                    (#params)
                });
            } else {
                return_params.extend(quote! {
                    #params
                });
            }
        } else {
            return_params.extend(quote! {
                ()
            });
        }

        output.extend(quote! {
            #message
            #function_name(#view #params) -> Result<#return_params, Error>;
        });

        output.extend(quote! {
            _blank_!();
        });
    }

    output
}

/// Assembles the TokenStream of the signature which is then added to the beginning of the file :)
fn signature() -> TokenStream {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let version = &format!("Generated with Sol2Ink v{VERSION}\n");
    let link = "https://github.com/Brushfam/sol2ink\n";
    quote! {
        _comment_!(#version);
        _comment_!(#link);
        _blank_!();
    }
}

/// Formats an expression provided with the casing provided
/// Appends `_is_rust_keyword` to identifiers which collide with rust keyword
fn format_expression(expression_raw: &String, case: Case) -> String {
    if expression_raw == "_" {
        return expression_raw.clone()
    }
    let desired_name = expression_raw.to_case(case);
    if RUST_KEYWORDS.contains(&desired_name.as_str()) {
        format!("{}_is_rust_keyword", &desired_name)
    } else {
        desired_name
    }
}

/// Returns the TokenStream of a `Type` enum variant
impl ToTokens for Type {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.extend(match self {
            Type::AccountId => quote!(AccountId),
            Type::Bool => quote!(bool),
            Type::String => quote!(String),
            Type::Int(size) => TokenStream::from_str(&format!("i{size}")).unwrap(),
            Type::Uint(size) => TokenStream::from_str(&format!("u{size}")).unwrap(),
            Type::Bytes(size) => TokenStream::from_str(&format!("[u8; {size}]")).unwrap(),
            Type::DynamicBytes => quote!(Vec<u8>),
            Type::Variable(name) => {
                TokenStream::from_str(&format_expression(name, Pascal)).unwrap()
            }
            Type::Mapping(keys, value) => {
                if keys.len() == 1 {
                    let key = &keys[0];
                    quote!( Mapping <#key, #value>)
                } else {
                    quote!(Mapping <(#(#keys,)*), #value>)
                }
            }
            Type::Array(ty, _) => quote!(Vec< #ty >),
            Type::None => quote!(),
            Type::MemberAccess(from, identifeir) => {
                let parsed_identifier =
                    TokenStream::from_str(&format_expression(identifeir, Snake)).unwrap();
                quote!(#from :: #parsed_identifier)
            }
        })
    }
}

/// Returns the TokenStream of a `Statement` enum variant
impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Statement::Assembly => quote!(__comment__!("Assembly block here. Parsing assembly is not implemented yet");),
            Statement::Block(body) => quote!(#(#body)*),
            Statement::Break => quote!(break),
            Statement::Continue => quote!(continue),
            Statement::DoWhile(body, condition) => {
                quote!(
                    loop {
                        #body
                        if ! #condition {
                            break
                        }
                    }
                )
            }
            Statement::Emit(expression) => {
                match expression {
                    Expression::FunctionCall(identifier, args,_)
                    if let Expression::Variable(event_name,_,location)=*identifier.clone()=> {
                        let fn_name = TokenStream::from_str(&format!(
                            "_emit_{}",
                            &event_name.to_case(Snake)
                        ))
                        .unwrap();
                        quote!( #location . #fn_name ( #(#args),* ); )
                    }
                    _ => unreachable!("Emit can be only function call"),
                }
            }
            Statement::Error => todo!(),
            Statement::Expression(expression) => quote!(#expression;),
            Statement::For(declaration, condition, on_pass, body) => {
                quote!(
                    #declaration
                    while #condition {
                        #body
                        #on_pass
                    }
                )
            }
            Statement::If(condition, if_true, if_false) => {
                match if_false {
                    Some(statement) => {
                        match *statement.clone() {
                            Statement::If(..)=> quote!(
                                if #condition {
                                    #if_true
                                } else #if_false
                            ),
                            _ => quote!(if #condition {
                                #if_true
                            }else { #if_false}
                        )
                        }
                    },
                    None => quote!(if #condition {
                        #if_true
                    }),
                }
            }
            Statement::Return(expression) => quote!(return Ok(#expression)),
            Statement::Revert(reason, _) => {
                quote!( return Err( Error::Custom(String::from(#reason) )) )},
            Statement::RevertNamedArgs => todo!(),
            Statement::Try(expression) => {
                quote!(
                    if #expression .is_err() {
                        return Err(Error::Custom("Try failed"))
                    }
                )
            }
            Statement::UncheckedBlock(statements) => {
                quote!(
                    // _comment_!("Please handle unchecked blocks manually >>>");
                    #(#statements)*
                    // _comment_!("<<< Please handle unchecked blocks manually");
                )
            }
            Statement::VariableDefinition(definition, initial_value) => {
                if let Some(initial_value) = initial_value {
                    quote!( #definition = #initial_value; )
                } else {
                    quote!( #initial_value; )
                }
            }
            Statement::While(condition, body) => {
                quote!(
                    while #condition {
                        #body
                    }
                )
            }
        });
    }
}

/// Returns the TokenStream of an `Expression` enum variant
impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! transform_location {
            ($to_declare:ident, $location: expr) => {
                let $to_declare = match $location {
                    VariableAccessLocation::Constructor => quote!(instance.),
                    VariableAccessLocation::Modifier => quote!(T::),
                    VariableAccessLocation::Any => quote!(Self::),
                };
            }
        }
        tokens.extend(match self {
            Expression::Add(left, right) => quote!( #left + #right),
            Expression::ArraySubscript(expression, index) => {
                quote!( #expression [ #index ])
            }
            Expression::Assign(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (#mapping .insert(&(#(#indices),*), & (#value)) )
                    }
                    _ => quote!( #variable = #value ),
                }
            }
            Expression::AssignAdd(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (
                            let new_value = #mapping .get(&( #(#indices),* )).unwrap_or_default() + #value;
                            #mapping .insert(&(#(#indices),*), & new_value)
                        )
                    }
                    _ => quote!( #variable += #value ),
                }
            }
            Expression::AssignDivide(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (
                            let new_value = #mapping .get(&(#(#indices),*)).unwrap_or_default() / #value;
                            #mapping .insert(&(#(#indices),*), & new_value)
                        )
                    }
                    _ => quote!( #variable /= #value ),
                }
            }
            Expression::AssignModulo(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (
                            let new_value = #mapping .get(&(#(#indices),*)).unwrap_or_default() % #value;
                            #mapping .insert(&(#(#indices),*), & new_value)
                        )
                    }
                    _ => quote!( #variable %= #value ),
                }
            }
            Expression::AssignMultiply(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (
                            let new_value = #mapping .get(&(#(#indices),*)).unwrap_or_default() * #value;
                            #mapping .insert(&(#(#indices),*), & new_value)
                        )
                    }
                    _ => quote!( #variable *= #value ),
                }
            }
            Expression::AssignSubtract(variable, value) => {
                match *variable.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (
                            let new_value = #mapping .get(&(#(#indices),*)).unwrap_or_default() - #value;
                            #mapping .insert(&(#(#indices),*), & new_value)
                        )
                    }
                    _ => quote!( #variable -= #value ),
                }
            }
            Expression::BoolLiteral(value) => {
                quote!(#value)
            }
            Expression::Delete(expression) => {
                match *expression.clone() {
                    Expression::MappingSubscript(mapping, indices) => {
                        quote! (#mapping .remove(&(#(#indices),*)) )
                    }
                    _ => quote!(_comment_!("Deletion of storage member")),
                }
            }
            Expression::Divide(left, right) => quote!( #left / #right ),
            Expression::Equal(left, right) => {
                quote!(
                    #left == #right
                )
            }
            Expression::FunctionCall(function, args,value) => {
                match *function.clone() {
                    Expression::MemberAccess(_,name)if name == "decode" =>{
                        quote!(  #function ( __comment__!(#(#args),*) )? )
                    }
                    Expression::Variable(name, ..) if name == "require" => {
                        let condition = &args[0];
                        if args.len() > 1 {
                            let error = &args[1];
                            match error {
                                Expression::Variable(..) => {
                                    quote!(
                                        if ! (#condition) {
                                            return Err(Error::Custom( #error ))
                                        }
                                    )
                                }
                                _ => {
                                    quote!(
                                        if ! (#condition) {
                                            return Err(Error::Custom( String::from( #error) ))
                                        }
                                    )
                                }
                            }
                        } else {
                            quote!(
                                if ! (#condition)   {
                                    return Err(Error::Custom( String::from("No error message provdided :)") ))
                                }
                            )
                        }
                    }
                    Expression::Type(ty) if let Type::AccountId = *ty.clone() => {
                        if args.len() > 1 {
                            unreachable!("Multiple parameters were provided to `address` call")
                        }
                        let account_id = &args[0];
                        match account_id {
                            Expression::NumberLiteral(number) if number == "0" => {
                                quote!(ZERO_ADDRESS.into())
                            }
                            Expression::This(location) => {
                                let location = match location {
                                    VariableAccessLocation::Constructor => quote!(instance.),
                                    VariableAccessLocation::Modifier => quote!(T::),
                                    VariableAccessLocation::Any => quote!(Self::),
                                };
                                quote!( #location env().account_id() )
                            }
                            _ => quote!( AccountId::from(#account_id) ),
                        }
                    }
                    Expression::Type(ty) => {
                        match *ty {
                            Type::DynamicBytes => quote!( Vec::<u8>::from ( #(#args),* ) ),
                            _ => quote!( <#ty> :: from ( #(#args),* ) ),
                        }
                    }
                    Expression::Variable(name, ..) if name == "type" => {
                        quote!(
                            type_of ( #(#args),* )?
                        )
                    }
                    _ if let Some(value) = value => {
                        quote!(
                            #function ( #(#args),* ).transferred_value( #value )?
                        )
                    }
                    _ => {
                        quote!(
                            #function ( #(#args),* )?
                        )
                    }
                }
            }
            Expression::Less(left, right) => {
                quote!( #left < #right )
            }
            Expression::LessEqual(left, right) => {
                quote!( #left <= #right )
            }
            Expression::List(list) => quote!( (#(#list),*) ),
            Expression::MappingSubscript(array, indices) => {
                if indices.len() > 1 {
                    quote! (#array.get(&(#(#indices),*)).unwrap_or_default())
                } else {
                    quote! (#array.get(&#(#indices),*).unwrap_or_default())
                }
            }
            Expression::MemberAccess(left, member) => {
                match *left.clone() {
                    Expression::Variable(name, _,location) if name == "msg" => {
                        transform_location!(location, location);
                        match member.as_str() {
                            "sender" => quote!(#location env().caller()),
                            "value" => quote!(#location env().transferred_value()),
                            "gas" => quote!(#location env().gas_left()),
                            "sig" => todo!("Function selector (msg.sig)"),
                            "data" => todo!("Function data (msg.data)"),
                            _ => panic!("msg.{member} is not implemented!"),
                        }
                    }
                    Expression::Variable(name, _,location) if name == "block" => {
                        transform_location!(location, location);
                        match member.as_str() {
                            "number" => quote!(#location env().block_number()),
                            "timestamp" => quote!(#location env().block_timestamp()),
                            "basefee" => todo!("block.basefee"),
                            "chainid" => todo!("block.chainid"),
                            "coinbase" => todo!("block.coinbase"),
                            "difficulty" => todo!("block.difficulty"),
                            "gaslimit" => todo!("block.gaslimit"),
                            _ => panic!("block.{member} is not implemented!"),
                        }
                    }
                    _ => {
                        let ident = TokenStream::from_str(&member.to_case(Snake)).unwrap();
                        quote!( #left . #ident)
                    }
                }
            }
            Expression::Modifier(name,args) => {
                let parsed_name = TokenStream::from_str( &format_expression(name, Snake)).unwrap();
                quote!( #[modifiers( #parsed_name ( #(#args),* ) )] )
            }
            Expression::ModifierBody => { quote!( body(instance) ) }
            Expression::Modulo(left, right) => quote!( #left % #right ),
            Expression::More(left, right) => {
                quote!( #left > #right )
            }
            Expression::MoreEqual(left, right) => {
                quote!( #left >= #right )
            }
            Expression::Multiply(left, right) => quote!( #left * #right ),
            Expression::New(new) => {
                match *new.clone() {
                    // new array
                    Expression::FunctionCall(array, values, _)
                        if let Expression::ArraySubscript(ty, _) = *array.clone() =>
                    {
                        quote!(vec!( #ty ::default(); #(#values)* ))
                    }
                    Expression::FunctionCall(bytes, size, _)
                        if let Expression::Type(ty) = *bytes.clone() =>
                    {
                        if let Type::DynamicBytes = *ty {
                            quote!( Vec::with_capacity(#(#size)*) )
                        }else {
                            todo!("{new:?}")
                        }
                    }
                    _ => todo!("{new:?}"),
                }
            }
            Expression::Not(expression) => {
                quote!( ! #expression )
            }
            Expression::NotEqual(left, right) => {
                quote!( #left != #right )
            }
            Expression::NumberLiteral(value) => TokenStream::from_str(value).unwrap(),
            Expression::Or(left, right) => {
                quote!(
                   #left || #right
                )
            }
            Expression::Parenthesis(expression) => {
                quote! ( (#expression) )
            }
            Expression::PostDecrement(expression) => {
                quote!(
                    #expression -= 1
                )
            }
            Expression::PostIncrement(expression) => {
                quote!(
                    #expression += 1
                )
            }
            Expression::Power(left, right) => quote!( #left .pow( #right ) ),
            Expression::PreDecrement(expression) => {
                quote!(
                    #expression -= 1
                )
            }
            Expression::PreIncrement(expression) => {
                quote!(
                    #expression += 1
                )
            }
            Expression::StringLiteral(strings) => {
                let joined = &strings.join(" ");
                quote!(#joined)
            }
            Expression::Subtract(left, right) => {
                quote!(
                    #left - #right
                )
            }
            Expression::Ternary(condition, if_true, if_false) => {
                quote!( if #condition { #if_true } else { #if_false } )
            }
            Expression::Type(ty) => quote!( #ty ),
            Expression::Variable(name, member_type,location) => {
                match member_type {
                    MemberType::Variable(_) => {
                        let formatted_name =TokenStream::from_str(&format_expression( name,Snake))
                            .unwrap();
                        if let VariableAccessLocation::Constructor = location {
                            quote!(#location.data. #formatted_name)
                        } else {
                            quote!(#location.data(). #formatted_name)
                        }
                    }
                    MemberType::Function => {
                        let formatted_name =TokenStream::from_str(&format_expression(name,Snake))
                            .unwrap();
                        quote!(#location.#formatted_name)
                    }
                    MemberType::FunctionPrivate => {
                        let formatted_name =TokenStream::from_str(&format!("_{}", name.to_case(Snake)))
                            .unwrap();
                        quote!(#location.#formatted_name)
                    }
                    MemberType::Constant => {
                        TokenStream::from_str(&format_expression(name, UpperSnake)).unwrap()
                    }
                    MemberType::None(_) => TokenStream::from_str(&format_expression(name, Snake)).unwrap(),
                }
            }
            Expression::VariableDeclaration(ty, name) => {
                let name = TokenStream::from_str(name).unwrap();
                quote!(let mut #name : #ty)
            }
            Expression::ShiftLeft(left, right) => {
                quote!(
                   #left << #right
                )
            }
            Expression::ShiftRight(left, right) => {
                quote!(
                   #left >> #right
                )
            }
            Expression::BitwiseAnd(left, right) => {
                quote!(
                   #left & #right
                )
            }
            Expression::BitwiseXor(left, right) => {
                quote!(
                   #left ^ #right
                )
            }
            Expression::BitwiseOr(left, right) => {
                quote!(
                   #left | #right
                )
            }
            Expression::AssignOr(left, right) => {
                quote!(
                   #left |= #right
                )
            }
            Expression::AssignAnd(left, right) => {
                quote!(
                   #left &= #right
                )
            }
            Expression::AssignXor(left, right) => {
                quote!(
                   #left ^= #right
                )
            }
            Expression::AssignShiftLeft(left, right) => {
                quote!(
                   #left <<= #right
                )
            }
            Expression::AssignShiftRight(left, right) => {
                quote!(
                   #left >>= #right
                )
            }
            Expression::HexLiteral(hex) => {
                quote!( &hex::decode(#hex) )
            }
            Expression::NamedFunctionCall(function, args) => {
                let names = args
                    .iter()
                    .map(|arg| {
                        let parsed = format_expression(&arg.0, Snake);
                        format_ident!("{parsed}")
                    })
                    .collect::<Vec<_>>();
                let values = args.iter().map(|arg| &arg.1).collect::<Vec<_>>();
                quote!( #function { #( #names : #values ),*  } )
            }
            Expression::And(left, right) => {
                quote!(
                   #left && #right
                )
            }
            Expression::ArrayLiteral(expressions) => quote!(vec![#(#expressions),*]),
            Expression::InvalidModifier(_, _) => quote!(),
            Expression::This(_) => quote!(),
            Expression::UnaryMinus(expression) => {
                quote!(
                    - #expression
                )
            }
            Expression::UnaryPlus(expression) => {
                quote!(
                    + #expression
                )
            }
            Expression::Unit(expression, unit) => {
               quote!( #expression * #unit )
            }
            Expression::ArraySlice(expression, start, end) => {
                quote!( #expression[#start..#end] )
            }
            Expression::None => quote!()
        })
    }
}

/// Returns the TokenStream of a `VariableAccessLocation` enum variant
impl ToTokens for VariableAccessLocation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            VariableAccessLocation::Constructor => quote!(instance),
            VariableAccessLocation::Modifier => quote!(instance),
            VariableAccessLocation::Any => quote!(self),
        })
    }
}

/// Returns the TokenStream of a `FunctionParam` enum variant
impl ToTokens for FunctionParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name =
            TokenStream::from_str(&format!("__{}", format_expression(&self.name, Snake))).unwrap();
        let ty = &self.param_type;
        tokens.extend(quote!(#name : #ty ))
    }
}

/// Returns the TokenStream of an `Import` enum variant
impl ToTokens for Import {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Import::ModifierDefinition => {
                quote!(
                    use openbrush::modifier_definition;
                )
            }
            Import::Modifiers => {
                quote!(
                    use openbrush::modifiers;
                )
            }
            Import::AccountId => {
                quote!(
                    pub use openbrush::traits::AccountId;
                )
            }
            Import::Mapping => {
                quote!(
                    pub use openbrush::storage::Mapping;
                )
            }
            Import::String => {
                quote!(
                    pub use openbrush::traits::String;
                )
            }
            Import::Vec => {
                quote!(
                    pub use ink::prelude::vec::*;
                )
            }
            Import::ZeroAddress => {
                quote!(
                    pub use openbrush::traits::ZERO_ADDRESS;
                    pub use openbrush::traits::AccountIdExt;
                )
            }
        })
    }
}
