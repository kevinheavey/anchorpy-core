use anchor_syn::idl as anchor_idl;
use derive_more::{Display, From, Into};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyString, PyTuple},
    PyTypeInfo,
};
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solders_macros::{common_methods, pyhash, richcmp_eq_only};
use solders_traits::{
    CommonMethods, PyBytesBincode, PyBytesGeneral, PyFromBytesBincode, PyFromBytesGeneral, PyHash,
    RichcmpEqualityOnly,
};

macro_rules! struct_boilerplate {
    ($name:ident) => {
        impl PyBytesBincode for $name {}
        impl PyBytesGeneral for $name {
            fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a pyo3::types::PyBytes {
                self.pybytes_bincode(py)
            }
        }
        impl PyFromBytesBincode<'_> for $name {}
        impl PyFromBytesGeneral for $name {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                Self::py_from_bytes_bincode(raw)
            }
        }
        impl RichcmpEqualityOnly for $name {}
        impl CommonMethods<'_> for $name {}
    };
}

macro_rules! debug_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

macro_rules! iter_into {
    ($obj:expr) => {
        $obj.into_iter().map(|x| x.into()).collect()
    };
}

fn to_py_value_err(err: &impl ToString) -> PyErr {
    PyValueError::new_err(err.to_string())
}

fn handle_py_value_err<T: Into<P>, E: ToString, P>(res: Result<T, E>) -> PyResult<P> {
    res.map_or_else(|e| Err(to_py_value_err(&e)), |v| Ok(v.into()))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, Hash)]
#[pyclass(module = "anchorpy_core.idl")]
pub enum IdlTypeSimple {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    PublicKey,
}

impl From<IdlTypeSimple> for anchor_idl::types::IdlType {
    fn from(t: IdlTypeSimple) -> Self {
        match t {
            IdlTypeSimple::Bool => Self::Bool,
            IdlTypeSimple::U8 => Self::U8,
            IdlTypeSimple::I8 => Self::I8,
            IdlTypeSimple::U16 => Self::U16,
            IdlTypeSimple::I16 => Self::I16,
            IdlTypeSimple::U32 => Self::U32,
            IdlTypeSimple::I32 => Self::I32,
            IdlTypeSimple::F32 => Self::F32,
            IdlTypeSimple::U64 => Self::U64,
            IdlTypeSimple::I64 => Self::I64,
            IdlTypeSimple::F64 => Self::F64,
            IdlTypeSimple::U128 => Self::U128,
            IdlTypeSimple::I128 => Self::I128,
            IdlTypeSimple::U256 => Self::U256,
            IdlTypeSimple::I256 => Self::I256,
            IdlTypeSimple::Bytes => Self::Bytes,
            IdlTypeSimple::String => Self::String,
            IdlTypeSimple::PublicKey => Self::PublicKey,
        }
    }
}

impl PyHash for IdlTypeSimple {}

#[pyhash]
#[pymethods]
impl IdlTypeSimple {}

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Serialize, Deserialize, Hash, Display)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefined(String);

impl PyHash for IdlTypeDefined {}

#[richcmp_eq_only]
#[common_methods]
#[pyhash]
#[pymethods]
impl IdlTypeDefined {
    #[new]
    pub fn new(defined: String) -> Self {
        defined.into()
    }

    #[getter]
    pub fn defined(&self) -> String {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeDefined);

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Serialize, Deserialize, Hash, Display)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeGeneric(String);

impl PyHash for IdlTypeGeneric {}

#[richcmp_eq_only]
#[common_methods]
#[pyhash]
#[pymethods]
impl IdlTypeGeneric {
    #[new]
    pub fn new(generic: String) -> Self {
        generic.into()
    }

    #[getter]
    pub fn generic(&self) -> String {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeGeneric);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromPyObject)]
#[serde(rename_all = "camelCase")]
pub enum IdlDefinedTypeArg {
    Generic(IdlTypeGeneric),
    Value(String),
    Type(IdlType),
}

impl IntoPy<PyObject> for IdlDefinedTypeArg {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Generic(s) => s.into_py(py),
            Self::Value(v) => v.into_py(py),
            Self::Type(t) => t.into_py(py),
        }
    }
}

impl From<IdlDefinedTypeArg> for anchor_idl::types::IdlDefinedTypeArg {
    fn from(value: IdlDefinedTypeArg) -> Self {
        match value {
            IdlDefinedTypeArg::Generic(s) => Self::Generic(s.0),
            IdlDefinedTypeArg::Value(s) => Self::Value(s),
            IdlDefinedTypeArg::Type(t) => Self::Type(t.into()),
        }
    }
}

impl From<anchor_idl::types::IdlDefinedTypeArg> for IdlDefinedTypeArg {
    fn from(value: anchor_idl::types::IdlDefinedTypeArg) -> Self {
        match value {
            anchor_idl::types::IdlDefinedTypeArg::Generic(s) => Self::Generic(IdlTypeGeneric(s)),
            anchor_idl::types::IdlDefinedTypeArg::Value(s) => Self::Value(s),
            anchor_idl::types::IdlDefinedTypeArg::Type(t) => Self::Type(t.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefinedWithTypeArgs {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    args: Vec<IdlDefinedTypeArg>,
}

debug_display!(IdlTypeDefinedWithTypeArgs);

#[pymethods]
impl IdlTypeDefinedWithTypeArgs {
    #[new]
    pub fn new(name: String, args: Vec<IdlDefinedTypeArg>) -> Self {
        Self { name, args }
    }
}

struct_boilerplate!(IdlTypeDefinedWithTypeArgs);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeOption(Box<IdlType>);

debug_display!(IdlTypeOption);

#[pymethods]
impl IdlTypeOption {
    #[new]
    pub fn new(option: IdlType) -> Self {
        Self(option.into())
    }

    #[getter]
    pub fn option(&self) -> IdlType {
        *self.0.clone()
    }
}

struct_boilerplate!(IdlTypeOption);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeVec(Box<IdlType>);

#[pymethods]
impl IdlTypeVec {
    #[new]
    pub fn new(vec: IdlType) -> Self {
        Self(vec.into())
    }

    #[getter]
    pub fn vec(&self) -> IdlType {
        *self.0.clone()
    }
}

struct_boilerplate!(IdlTypeVec);
debug_display!(IdlTypeVec);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeArray(Box<IdlType>, usize);

#[pymethods]
impl IdlTypeArray {
    #[new]
    pub fn new(array: (IdlType, usize)) -> Self {
        Self(array.0.into(), array.1)
    }

    #[getter]
    pub fn array(&self) -> (IdlType, usize) {
        (*self.0.clone(), self.1)
    }
}

struct_boilerplate!(IdlTypeArray);
debug_display!(IdlTypeArray);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeGenericLenArray(Box<IdlType>, String);

#[pymethods]
impl IdlTypeGenericLenArray {
    #[new]
    pub fn new(generic_len_array: (IdlType, String)) -> Self {
        Self(generic_len_array.0.into(), generic_len_array.1)
    }

    #[getter]
    pub fn generic_len_array(&self) -> (IdlType, String) {
        (*self.0.clone(), self.1.clone())
    }
}

struct_boilerplate!(IdlTypeGenericLenArray);
debug_display!(IdlTypeGenericLenArray);

#[derive(Debug, Clone, PartialEq, FromPyObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IdlTypeCompound {
    Defined(IdlTypeDefined),
    Option(IdlTypeOption),
    Vec(IdlTypeVec),
    Array(IdlTypeArray),
    GenericLenArray(IdlTypeGenericLenArray),
    Generic(IdlTypeGeneric),
    DefinedWithTypeArgs(IdlTypeDefinedWithTypeArgs),
}

impl From<IdlTypeCompound> for anchor_idl::types::IdlType {
    fn from(t: IdlTypeCompound) -> Self {
        match t {
            IdlTypeCompound::Defined(d) => Self::Defined(d.0),
            IdlTypeCompound::Option(o) => Self::Option(Box::new(Self::from(*o.0))),
            IdlTypeCompound::Vec(v) => Self::Vec(Box::new(Self::from(*v.0))),
            IdlTypeCompound::Array(a) => Self::Array(Box::new(Self::from(*a.0)), a.1),
            IdlTypeCompound::GenericLenArray(g) => {
                Self::GenericLenArray(Box::new(Self::from(*g.0)), g.1)
            }
            IdlTypeCompound::Generic(g) => Self::Generic(g.0),
            IdlTypeCompound::DefinedWithTypeArgs(d) => Self::DefinedWithTypeArgs {
                name: d.name,
                args: iter_into!(d.args),
            },
        }
    }
}

impl IntoPy<PyObject> for IdlTypeCompound {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlTypeCompound::Defined(x) => x.into_py(py),
            IdlTypeCompound::Option(x) => x.into_py(py),
            IdlTypeCompound::Vec(x) => x.into_py(py),
            IdlTypeCompound::Array(x) => x.into_py(py),
            IdlTypeCompound::GenericLenArray(x) => x.into_py(py),
            IdlTypeCompound::Generic(x) => x.into_py(py),
            IdlTypeCompound::DefinedWithTypeArgs(x) => x.into_py(py),
        }
    }
}

#[derive(FromPyObject, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlType {
    Simple(IdlTypeSimple),
    Compound(IdlTypeCompound),
}

impl IntoPy<PyObject> for IdlType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlType::Simple(s) => s.into_py(py),
            IdlType::Compound(c) => c.into_py(py),
        }
    }
}

impl From<anchor_idl::types::IdlType> for IdlType {
    fn from(t: anchor_idl::types::IdlType) -> Self {
        match t {
            anchor_idl::types::IdlType::Bool => Self::Simple(IdlTypeSimple::Bool),
            anchor_idl::types::IdlType::U8 => Self::Simple(IdlTypeSimple::U8),
            anchor_idl::types::IdlType::I8 => Self::Simple(IdlTypeSimple::I8),
            anchor_idl::types::IdlType::U16 => Self::Simple(IdlTypeSimple::U16),
            anchor_idl::types::IdlType::I16 => Self::Simple(IdlTypeSimple::I16),
            anchor_idl::types::IdlType::U32 => Self::Simple(IdlTypeSimple::U32),
            anchor_idl::types::IdlType::I32 => Self::Simple(IdlTypeSimple::I32),
            anchor_idl::types::IdlType::F32 => Self::Simple(IdlTypeSimple::F32),
            anchor_idl::types::IdlType::U64 => Self::Simple(IdlTypeSimple::U64),
            anchor_idl::types::IdlType::I64 => Self::Simple(IdlTypeSimple::I64),
            anchor_idl::types::IdlType::F64 => Self::Simple(IdlTypeSimple::F64),
            anchor_idl::types::IdlType::U128 => Self::Simple(IdlTypeSimple::U128),
            anchor_idl::types::IdlType::I128 => Self::Simple(IdlTypeSimple::I128),
            anchor_idl::types::IdlType::U256 => Self::Simple(IdlTypeSimple::U256),
            anchor_idl::types::IdlType::I256 => Self::Simple(IdlTypeSimple::I256),
            anchor_idl::types::IdlType::Bytes => Self::Simple(IdlTypeSimple::Bytes),
            anchor_idl::types::IdlType::String => Self::Simple(IdlTypeSimple::String),
            anchor_idl::types::IdlType::PublicKey => Self::Simple(IdlTypeSimple::PublicKey),
            anchor_idl::types::IdlType::Defined(d) => {
                Self::Compound(IdlTypeCompound::Defined(IdlTypeDefined(d)))
            }
            anchor_idl::types::IdlType::Option(o) => Self::Compound(IdlTypeCompound::Option(
                IdlTypeOption(Box::new(IdlType::from(*o))),
            )),
            anchor_idl::types::IdlType::Vec(v) => Self::Compound(IdlTypeCompound::Vec(IdlTypeVec(
                Box::new(IdlType::from(*v)),
            ))),
            anchor_idl::types::IdlType::Array(a, size) => Self::Compound(IdlTypeCompound::Array(
                IdlTypeArray(Box::new(IdlType::from(*a)), size),
            )),
            anchor_idl::types::IdlType::GenericLenArray(type_, generic) => {
                Self::Compound(IdlTypeCompound::GenericLenArray(IdlTypeGenericLenArray(
                    Box::new(IdlType::from(*type_)),
                    generic,
                )))
            }
            anchor_idl::types::IdlType::Generic(g) => {
                Self::Compound(IdlTypeCompound::Generic(IdlTypeGeneric(g)))
            }
            anchor_idl::types::IdlType::DefinedWithTypeArgs { name, args } => {
                Self::Compound(IdlTypeCompound::DefinedWithTypeArgs(
                    IdlTypeDefinedWithTypeArgs::new(name, iter_into!(args)),
                ))
            }
        }
    }
}

impl From<IdlType> for anchor_idl::types::IdlType {
    fn from(t: IdlType) -> Self {
        match t {
            IdlType::Simple(s) => Self::from(s),
            IdlType::Compound(c) => Self::from(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlConst(anchor_idl::types::IdlConst);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlConst {
    #[new]
    pub fn new(name: String, ty: IdlType, value: String) -> Self {
        anchor_idl::types::IdlConst {
            name,
            ty: ty.into(),
            value,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn value(&self) -> String {
        self.0.value.clone()
    }
}

struct_boilerplate!(IdlConst);
debug_display!(IdlConst);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlField(anchor_idl::types::IdlField);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlField {
    #[new]
    pub fn new(name: String, docs: Option<Vec<String>>, ty: IdlType) -> Self {
        anchor_idl::types::IdlField {
            name,
            docs,
            ty: ty.into(),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Option<Vec<String>> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }
}

struct_boilerplate!(IdlField);
debug_display!(IdlField);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefinitionTyStruct(Vec<IdlField>);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlTypeDefinitionTyStruct {
    #[new]
    pub fn new(fields: Vec<IdlField>) -> Self {
        fields.into()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlField> {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeDefinitionTyStruct);
debug_display!(IdlTypeDefinitionTyStruct);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefinitionTyAlias(IdlType);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlTypeDefinitionTyAlias {
    #[new]
    pub fn new(value: IdlType) -> Self {
        Self(value)
    }

    #[getter]
    pub fn value(&self) -> IdlType {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeDefinitionTyAlias);
debug_display!(IdlTypeDefinitionTyAlias);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct EnumFieldsNamed(Vec<IdlField>);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EnumFieldsNamed {
    #[new]
    pub fn new(fields: Vec<IdlField>) -> Self {
        fields.into()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlField> {
        self.0.clone()
    }
}

struct_boilerplate!(EnumFieldsNamed);
debug_display!(EnumFieldsNamed);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct EnumFieldsTuple(Vec<IdlType>);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl EnumFieldsTuple {
    #[new]
    pub fn new(fields: Vec<IdlType>) -> Self {
        fields.into()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlType> {
        self.0.clone()
    }
}

struct_boilerplate!(EnumFieldsTuple);
debug_display!(EnumFieldsTuple);

#[derive(Debug, Clone, PartialEq, FromPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumFields {
    Named(EnumFieldsNamed),
    Tuple(EnumFieldsTuple),
}

impl From<EnumFields> for anchor_idl::types::EnumFields {
    fn from(t: EnumFields) -> Self {
        match t {
            EnumFields::Named(n) => Self::Named(iter_into!(n.0)),
            EnumFields::Tuple(t) => Self::Tuple(iter_into!(t.0)),
        }
    }
}

impl From<anchor_idl::types::EnumFields> for EnumFields {
    fn from(t: anchor_idl::types::EnumFields) -> Self {
        match t {
            anchor_idl::types::EnumFields::Named(n) => Self::Named(EnumFieldsNamed(iter_into!(n))),
            anchor_idl::types::EnumFields::Tuple(t) => Self::Tuple(EnumFieldsTuple(iter_into!(t))),
        }
    }
}

impl IntoPy<PyObject> for EnumFields {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            EnumFields::Named(x) => x.into_py(py),
            EnumFields::Tuple(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlEnumVariant(anchor_idl::types::IdlEnumVariant);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlEnumVariant {
    #[new]
    pub fn new(name: String, fields: Option<EnumFields>) -> Self {
        anchor_idl::types::IdlEnumVariant {
            name,
            fields: fields.map(|f| f.into()),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn fields(&self) -> Option<EnumFields> {
        self.0.fields.clone().map(|f| f.into())
    }
}

struct_boilerplate!(IdlEnumVariant);
debug_display!(IdlEnumVariant);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefinitionTyEnum(Vec<IdlEnumVariant>);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlTypeDefinitionTyEnum {
    #[new]
    pub fn new(variants: Vec<IdlEnumVariant>) -> Self {
        variants.into()
    }

    #[getter]
    pub fn variants(&self) -> Vec<IdlEnumVariant> {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeDefinitionTyEnum);
debug_display!(IdlTypeDefinitionTyEnum);

#[derive(Debug, Clone, PartialEq, FromPyObject, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct(IdlTypeDefinitionTyStruct),
    Enum(IdlTypeDefinitionTyEnum),
    Alias(IdlTypeDefinitionTyAlias),
}

impl From<IdlTypeDefinitionTy> for anchor_idl::types::IdlTypeDefinitionTy {
    fn from(t: IdlTypeDefinitionTy) -> Self {
        match t {
            IdlTypeDefinitionTy::Struct(s) => Self::Struct {
                fields: iter_into!(s.0),
            },
            IdlTypeDefinitionTy::Enum(e) => Self::Enum {
                variants: iter_into!(e.0),
            },
            IdlTypeDefinitionTy::Alias(a) => Self::Alias { value: a.0.into() },
        }
    }
}

impl From<anchor_idl::types::IdlTypeDefinitionTy> for IdlTypeDefinitionTy {
    fn from(t: anchor_idl::types::IdlTypeDefinitionTy) -> Self {
        match t {
            anchor_idl::types::IdlTypeDefinitionTy::Struct { fields } => {
                Self::Struct(IdlTypeDefinitionTyStruct(iter_into!(fields)))
            }
            anchor_idl::types::IdlTypeDefinitionTy::Enum { variants } => {
                Self::Enum(IdlTypeDefinitionTyEnum(iter_into!(variants)))
            }
            anchor_idl::types::IdlTypeDefinitionTy::Alias { value } => {
                Self::Alias(IdlTypeDefinitionTyAlias(value.into()))
            }
        }
    }
}

impl IntoPy<PyObject> for IdlTypeDefinitionTy {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlTypeDefinitionTy::Struct(x) => x.into_py(py),
            IdlTypeDefinitionTy::Enum(x) => x.into_py(py),
            IdlTypeDefinitionTy::Alias(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeDefinition(anchor_idl::types::IdlTypeDefinition);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlTypeDefinition {
    #[new]
    pub fn new(
        name: String,
        docs: Option<Vec<String>>,
        ty: IdlTypeDefinitionTy,
        generics: Option<Vec<String>>,
    ) -> Self {
        anchor_idl::types::IdlTypeDefinition {
            name,
            docs,
            ty: ty.into(),
            generics,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Option<Vec<String>> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlTypeDefinitionTy {
        self.0.ty.clone().into()
    }
}

struct_boilerplate!(IdlTypeDefinition);
debug_display!(IdlTypeDefinition);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromPyObject)]
#[serde(untagged)]
pub enum IdlAccountItem {
    IdlAccount(IdlAccount),
    IdlAccounts(IdlAccounts),
}

impl From<IdlAccountItem> for anchor_idl::types::IdlAccountItem {
    fn from(a: IdlAccountItem) -> Self {
        match a {
            IdlAccountItem::IdlAccount(x) => Self::IdlAccount(x.into()),
            IdlAccountItem::IdlAccounts(x) => Self::IdlAccounts(x.into()),
        }
    }
}

impl From<anchor_idl::types::IdlAccountItem> for IdlAccountItem {
    fn from(a: anchor_idl::types::IdlAccountItem) -> Self {
        match a {
            anchor_idl::types::IdlAccountItem::IdlAccount(x) => Self::IdlAccount(x.into()),
            anchor_idl::types::IdlAccountItem::IdlAccounts(x) => Self::IdlAccounts(x.into()),
        }
    }
}

impl IntoPy<PyObject> for IdlAccountItem {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlAccountItem::IdlAccount(x) => x.into_py(py),
            IdlAccountItem::IdlAccounts(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlAccounts(anchor_idl::types::IdlAccounts);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlAccounts {
    #[new]
    pub fn new(name: String, accounts: Vec<IdlAccountItem>) -> Self {
        anchor_idl::types::IdlAccounts {
            name,
            accounts: iter_into!(accounts),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn accounts(&self) -> Vec<IdlAccountItem> {
        iter_into!(self.0.accounts.clone())
    }
}

struct_boilerplate!(IdlAccounts);
debug_display!(IdlAccounts);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlSeedConst(anchor_idl::types::IdlSeedConst);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlSeedConst {
    #[new]
    pub fn new(ty: IdlType, value: &PyAny) -> PyResult<Self> {
        let parsed_val = handle_py_value_err(depythonize::<Value>(value))?;
        Ok(anchor_idl::types::IdlSeedConst {
            ty: ty.into(),
            value: parsed_val,
        }
        .into())
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn value(&self, py: Python<'_>) -> PyResult<PyObject> {
        handle_py_value_err(pythonize(py, &self.0.value))
    }
}

struct_boilerplate!(IdlSeedConst);
debug_display!(IdlSeedConst);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlSeedArg(anchor_idl::types::IdlSeedArg);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlSeedArg {
    #[new]
    pub fn new(ty: IdlType, path: String) -> Self {
        anchor_idl::types::IdlSeedArg {
            ty: ty.into(),
            path,
        }
        .into()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn path(&self) -> String {
        self.0.path.clone()
    }
}

struct_boilerplate!(IdlSeedArg);
debug_display!(IdlSeedArg);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlSeedAccount(anchor_idl::types::IdlSeedAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlSeedAccount {
    #[new]
    pub fn new(ty: IdlType, account: Option<String>, path: String) -> Self {
        anchor_idl::types::IdlSeedAccount {
            ty: ty.into(),
            account,
            path,
        }
        .into()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn acount(&self) -> Option<String> {
        self.0.account.clone()
    }

    #[getter]
    pub fn path(&self) -> String {
        self.0.path.clone()
    }
}

struct_boilerplate!(IdlSeedAccount);
debug_display!(IdlSeedAccount);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromPyObject)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum IdlSeed {
    Const(IdlSeedConst),
    Arg(IdlSeedArg),
    Account(IdlSeedAccount),
}

impl From<IdlSeed> for anchor_idl::types::IdlSeed {
    fn from(s: IdlSeed) -> Self {
        match s {
            IdlSeed::Const(x) => Self::Const(x.into()),
            IdlSeed::Arg(x) => Self::Arg(x.into()),
            IdlSeed::Account(x) => Self::Account(x.into()),
        }
    }
}

impl From<anchor_idl::types::IdlSeed> for IdlSeed {
    fn from(s: anchor_idl::types::IdlSeed) -> Self {
        match s {
            anchor_idl::types::IdlSeed::Const(x) => Self::Const(x.into()),
            anchor_idl::types::IdlSeed::Arg(x) => Self::Arg(x.into()),
            anchor_idl::types::IdlSeed::Account(x) => Self::Account(x.into()),
        }
    }
}

impl IntoPy<PyObject> for IdlSeed {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlSeed::Const(x) => x.into_py(py),
            IdlSeed::Arg(x) => x.into_py(py),
            IdlSeed::Account(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlPda(anchor_idl::types::IdlPda);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlPda {
    #[new]
    pub fn new(seeds: Vec<IdlSeed>, program_id: Option<IdlSeed>) -> Self {
        anchor_idl::types::IdlPda {
            seeds: iter_into!(seeds),
            program_id: program_id.map(|x| x.into()),
        }
        .into()
    }

    #[getter]
    pub fn seeds(&self) -> Vec<IdlSeed> {
        iter_into!(self.0.seeds.clone())
    }

    #[getter]
    pub fn program_id(&self) -> Option<IdlSeed> {
        self.0.program_id.clone().map(|x| x.into())
    }
}

struct_boilerplate!(IdlPda);
debug_display!(IdlPda);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlAccount(anchor_idl::types::IdlAccount);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlAccount {
    #[new]
    pub fn new(
        name: String,
        is_mut: bool,
        is_signer: bool,
        is_optional: Option<bool>,
        docs: Option<Vec<String>>,
        pda: Option<IdlPda>,
        relations: Vec<String>,
    ) -> Self {
        anchor_idl::types::IdlAccount {
            name,
            is_mut,
            is_signer,
            is_optional,
            docs,
            pda: pda.map(|x| x.into()),
            relations,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn is_mut(&self) -> bool {
        self.0.is_mut
    }

    #[getter]
    pub fn is_signer(&self) -> bool {
        self.0.is_signer
    }

    #[getter]
    pub fn is_optional(&self) -> Option<bool> {
        self.0.is_optional
    }

    #[getter]
    pub fn docs(&self) -> Option<Vec<String>> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn pda(&self) -> Option<IdlPda> {
        self.0.pda.clone().map(|x| x.into())
    }

    #[getter]
    pub fn relations(&self) -> Vec<String> {
        self.0.relations.clone()
    }
}

struct_boilerplate!(IdlAccount);
debug_display!(IdlAccount);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlInstruction(anchor_idl::types::IdlInstruction);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlInstruction {
    #[new]
    pub fn new(
        name: String,
        docs: Option<Vec<String>>,
        accounts: Vec<IdlAccountItem>,
        args: Vec<IdlField>,
        returns: Option<IdlType>,
    ) -> Self {
        anchor_idl::types::IdlInstruction {
            name,
            docs,
            accounts: iter_into!(accounts),
            args: iter_into!(args),
            returns: returns.map(|x| x.into()),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Option<Vec<String>> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn accounts(&self) -> Vec<IdlAccountItem> {
        iter_into!(self.0.accounts.clone())
    }

    #[getter]
    pub fn args(&self) -> Vec<IdlField> {
        iter_into!(self.0.args.clone())
    }

    #[getter]
    pub fn returns(&self) -> Option<IdlType> {
        self.0.returns.clone().map(|x| x.into())
    }
}

struct_boilerplate!(IdlInstruction);
debug_display!(IdlInstruction);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlState(anchor_idl::types::IdlState);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlState {
    #[new]
    pub fn new(strct: IdlTypeDefinition, methods: Vec<IdlInstruction>) -> Self {
        anchor_idl::types::IdlState {
            strct: strct.into(),
            methods: iter_into!(methods),
        }
        .into()
    }

    #[getter]
    pub fn strct(&self) -> IdlTypeDefinition {
        self.0.strct.clone().into()
    }

    #[getter]
    pub fn methods(&self) -> Vec<IdlInstruction> {
        iter_into!(self.0.methods.clone())
    }
}

struct_boilerplate!(IdlState);
debug_display!(IdlState);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlEvent(anchor_idl::types::IdlEvent);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlEvent {
    #[new]
    pub fn new(name: String, fields: Vec<IdlEventField>) -> Self {
        anchor_idl::types::IdlEvent {
            name,
            fields: iter_into!(fields),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlEventField> {
        iter_into!(self.0.fields.clone())
    }
}

struct_boilerplate!(IdlEvent);
debug_display!(IdlEvent);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlEventField(anchor_idl::types::IdlEventField);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlEventField {
    #[new]
    pub fn new(name: String, ty: IdlType, index: bool) -> Self {
        anchor_idl::types::IdlEventField {
            name,
            ty: ty.into(),
            index,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn index(&self) -> bool {
        self.0.index
    }
}

struct_boilerplate!(IdlEventField);
debug_display!(IdlEventField);

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlErrorCode(anchor_idl::types::IdlErrorCode);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl IdlErrorCode {
    #[new]
    pub fn new(code: u32, name: String, msg: Option<String>) -> Self {
        anchor_idl::types::IdlErrorCode { code, name, msg }.into()
    }

    #[getter]
    pub fn code(&self) -> u32 {
        self.0.code
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn msg(&self) -> Option<String> {
        self.0.msg.clone()
    }
}

struct_boilerplate!(IdlErrorCode);
debug_display!(IdlErrorCode);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct Idl(anchor_idl::types::Idl);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl Idl {
    #[allow(clippy::too_many_arguments)]
    #[new]
    pub fn new(
        version: String,
        name: String,
        docs: Option<Vec<String>>,
        constants: Vec<IdlConst>,
        instructions: Vec<IdlInstruction>,
        accounts: Vec<IdlTypeDefinition>,
        types: Vec<IdlTypeDefinition>,
        events: Option<Vec<IdlEvent>>,
        errors: Option<Vec<IdlErrorCode>>,
        metadata: &PyAny,
    ) -> PyResult<Self> {
        let parsed_metadata = handle_py_value_err(depythonize::<Value>(metadata))?;
        Ok(anchor_idl::types::Idl {
            version,
            name,
            docs,
            constants: iter_into!(constants),
            instructions: iter_into!(instructions),
            accounts: iter_into!(accounts),
            types: iter_into!(types),
            events: events.map(|x| iter_into!(x)),
            errors: errors.map(|x| iter_into!(x)),
            metadata: parsed_metadata,
        }
        .into())
    }

    #[getter]
    pub fn version(&self) -> String {
        self.0.version.clone()
    }
    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }
    #[getter]
    pub fn docs(&self) -> Option<Vec<String>> {
        self.0.docs.clone()
    }
    #[getter]
    pub fn constants(&self) -> Vec<IdlConst> {
        iter_into!(self.0.constants.clone())
    }
    #[getter]
    pub fn instructions(&self) -> Vec<IdlInstruction> {
        iter_into!(self.0.instructions.clone())
    }
    #[getter]
    pub fn accounts(&self) -> Vec<IdlTypeDefinition> {
        iter_into!(self.0.accounts.clone())
    }
    #[getter]
    pub fn types(&self) -> Vec<IdlTypeDefinition> {
        iter_into!(self.0.types.clone())
    }
    #[getter]
    pub fn events(&self) -> Option<Vec<IdlEvent>> {
        self.0.events.clone().map(|x| iter_into!(x))
    }
    #[getter]
    pub fn errors(&self) -> Option<Vec<IdlErrorCode>> {
        self.0.errors.clone().map(|x| iter_into!(x))
    }
    #[getter]
    pub fn metadata(&self, py: Python<'_>) -> PyResult<PyObject> {
        handle_py_value_err(pythonize(py, &self.0.metadata))
    }
}

struct_boilerplate!(Idl);
debug_display!(Idl);

pub(crate) fn create_idl_mod(py: Python) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "idl")?;
    m.add_class::<IdlTypeSimple>()?;
    m.add_class::<IdlTypeDefined>()?;
    m.add_class::<IdlTypeOption>()?;
    m.add_class::<IdlTypeVec>()?;
    m.add_class::<IdlTypeArray>()?;
    m.add_class::<IdlTypeGenericLenArray>()?;
    m.add_class::<IdlTypeDefinedWithTypeArgs>()?;
    m.add_class::<IdlConst>()?;
    m.add_class::<IdlField>()?;
    m.add_class::<IdlTypeDefinitionTyStruct>()?;
    m.add_class::<EnumFieldsNamed>()?;
    m.add_class::<EnumFieldsTuple>()?;
    m.add_class::<IdlEnumVariant>()?;
    m.add_class::<IdlTypeDefinitionTyEnum>()?;
    m.add_class::<IdlTypeDefinitionTyAlias>()?;
    m.add_class::<IdlTypeDefinition>()?;
    m.add_class::<IdlAccounts>()?;
    m.add_class::<IdlSeedConst>()?;
    m.add_class::<IdlSeedArg>()?;
    m.add_class::<IdlSeedAccount>()?;
    m.add_class::<IdlPda>()?;
    m.add_class::<IdlAccount>()?;
    m.add_class::<IdlInstruction>()?;
    m.add_class::<IdlState>()?;
    m.add_class::<IdlEvent>()?;
    m.add_class::<IdlEventField>()?;
    m.add_class::<IdlErrorCode>()?;
    m.add_class::<IdlTypeGeneric>()?;
    m.add_class::<Idl>()?;

    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let idl_account_item_members = vec![IdlAccount::type_object(py), IdlAccounts::type_object(py)];
    m.add(
        "IdlAccountItem",
        union.get_item(PyTuple::new(py, idl_account_item_members))?,
    )?;
    let idl_type_definition_ty_members = vec![
        IdlTypeDefinitionTyStruct::type_object(py),
        IdlTypeDefinitionTyEnum::type_object(py),
    ];
    m.add(
        "IdlTypeDefinitionTy",
        union.get_item(PyTuple::new(py, idl_type_definition_ty_members))?,
    )?;
    let idl_seed_members = vec![
        IdlSeedConst::type_object(py),
        IdlSeedArg::type_object(py),
        IdlSeedAccount::type_object(py),
    ];
    m.add(
        "IdlSeed",
        union.get_item(PyTuple::new(py, idl_seed_members))?,
    )?;
    let compound_members = vec![
        IdlTypeDefined::type_object(py),
        IdlTypeOption::type_object(py),
        IdlTypeVec::type_object(py),
        IdlTypeArray::type_object(py),
        IdlTypeDefinedWithTypeArgs::type_object(py),
        IdlTypeGenericLenArray::type_object(py),
    ];
    m.add(
        "IdlTypeCompound",
        union.get_item(PyTuple::new(py, compound_members.clone()))?,
    )?;
    let mut idl_type_members = vec![IdlTypeSimple::type_object(py)];
    idl_type_members.extend(compound_members);
    m.add(
        "IdlType",
        union.get_item(PyTuple::new(py, idl_type_members.clone()))?,
    )?;
    let mut idl_defined_type_arg_members = idl_type_members;
    idl_defined_type_arg_members.extend(vec![
        IdlTypeGeneric::type_object(py),
        PyString::type_object(py),
    ]);
    m.add(
        "IdlDefinedTypeArg",
        union.get_item(PyTuple::new(py, idl_defined_type_arg_members))?,
    )?;
    let enum_fields_members = vec![
        EnumFieldsNamed::type_object(py),
        EnumFieldsTuple::type_object(py),
    ];
    m.add(
        "EnumFields",
        union.get_item(PyTuple::new(py, enum_fields_members))?,
    )?;
    Ok(m)
}
