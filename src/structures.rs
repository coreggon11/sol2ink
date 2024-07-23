#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberType {
    StorageField(String),
    Function(FunctionHeader, String),
}

#[derive(Clone, Default, Debug)]
pub struct Contract {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub slots: Vec<StorageSlot>,
    pub constructor: Function,
    pub functions: Vec<Function>,
    pub modifiers: Vec<Function>,
    pub base: Vec<String>,
    pub is_abstract: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StorageSlot {
    pub name: String,
    pub fields: Vec<String>,
}

impl StorageSlot {
    pub fn new(name: String, fields: Vec<String>) -> Self {
        Self { name, fields }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Library {
    pub name: String,
    pub functions: Vec<Function>,
}

#[derive(Clone, Default, Debug)]
pub struct Interface {
    pub name: String,
    pub function_headers: Vec<FunctionHeader>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractField {
    pub name: String,
}

#[derive(Default, Clone, Debug)]
pub struct Function {
    pub header: FunctionHeader,
    pub calls: Vec<Call>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct FunctionHeader {
    pub name: String,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub modifiers: Vec<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Call {
    Read(CallType, String, String), // call type, contract name, call name
    ReadStorage(CallType, String, String), // call type, contract name, call name
    Write(CallType, String, String), // call type, contract name, call name
    WriteStorage(CallType, String, String), // call type, contract name, call name
    Library(String, String),        // Library, function
}

impl Call {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Call::Read(call_type, contract, calling)
            | Call::ReadStorage(call_type, contract, calling)
            | Call::WriteStorage(call_type, contract, calling)
            | Call::Write(call_type, contract, calling) => {
                format!(
                    "{}_{contract}_{calling}",
                    if call_type == &CallType::CallingFunction {
                        "f"
                    } else {
                        "s"
                    },
                )
            }
            _ => unreachable!("Must be remapped"),
        }
    }

    pub fn change_contract(&self, new_contract: &str) -> Self {
        match self.clone() {
            Call::Read(call_type, _, calling) => {
                Call::Read(call_type, new_contract.to_string(), calling)
            }
            Call::ReadStorage(call_type, _, calling) => {
                Call::ReadStorage(call_type, new_contract.to_string(), calling)
            }
            Call::Write(call_type, _, calling) => {
                Call::Write(call_type, new_contract.to_string(), calling)
            }
            Call::WriteStorage(call_type, _, calling) => {
                Call::WriteStorage(call_type, new_contract.to_string(), calling)
            }
            _ => unreachable!("Must be remapped"),
        }
    }

    pub fn is_read_storage(&self) -> bool {
        matches!(self, Call::ReadStorage(_, _, _))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CallType {
    CallingStorage,
    CallingStoragePointer,
    CallingFunction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Modifier(String, Vec<Call>),
}
