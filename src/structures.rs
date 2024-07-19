#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberType {
    StorageField(String),
    Function(FunctionHeader, String),
}

#[derive(Clone, Default, Debug)]
pub struct Contract {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub constructor: Function,
    pub functions: Vec<Function>,
    pub modifiers: Vec<Function>,
    pub base: Vec<String>,
    pub is_abstract: bool,
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
    Read(String),
    ReadStorage(String),
    Write(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Modifier(String, Vec<Call>),
}
