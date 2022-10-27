use anchor_syn::idl as anchor_idl;
use derive_more::{Display, From, Into};
use pyo3::{prelude::*, types::PyTuple, PyTypeInfo};
use serde::{Deserialize, Serialize};
use solders::{
    CommonMethods, PyBytesBincode, PyBytesGeneral, PyFromBytesBincode, PyFromBytesGeneral, PyHash,
    RichcmpEqualityOnly,
};
use solders_macros::{common_methods, pyhash, richcmp_eq_only};

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
    Bytes,
    String,
    PublicKey,
}

impl From<IdlTypeSimple> for anchor_idl::IdlType {
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
            IdlTypeSimple::Bytes => Self::Bytes,
            IdlTypeSimple::String => Self::String,
            IdlTypeSimple::PublicKey => Self::PublicKey,
        }
    }
}

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
    fn new(defined: String) -> Self {
        defined.into()
    }

    #[getter]
    pub fn defined(&self) -> String {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeDefined);

#[derive(Debug, Clone, PartialEq, From, Into, Serialize, Deserialize)]
#[pyclass(module = "anchorpy_core.idl", subclass)]
pub struct IdlTypeOption(Box<IdlType>);

debug_display!(IdlTypeOption);

#[pymethods]
impl IdlTypeOption {
    #[new]
    fn new(option: IdlType) -> Self {
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
    fn new(array: (IdlType, usize)) -> Self {
        Self(array.0.into(), array.1)
    }
}

struct_boilerplate!(IdlTypeArray);
debug_display!(IdlTypeArray);

#[derive(Debug, Clone, PartialEq, FromPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlTypeCompound {
    Defined(IdlTypeDefined),
    Option(IdlTypeOption),
    Vec(IdlTypeVec),
    Array(IdlTypeArray),
}

impl From<IdlTypeCompound> for anchor_idl::IdlType {
    fn from(t: IdlTypeCompound) -> Self {
        match t {
            IdlTypeCompound::Defined(d) => Self::Defined(d.0),
            IdlTypeCompound::Option(o) => Self::Option(Box::new(Self::from(*o.0))),
            IdlTypeCompound::Vec(v) => Self::Vec(Box::new(Self::from(*v.0))),
            IdlTypeCompound::Array(a) => Self::Array(Box::new(Self::from(*a.0)), a.1),
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

impl From<anchor_idl::IdlType> for IdlType {
    fn from(t: anchor_idl::IdlType) -> Self {
        match t {
            anchor_idl::IdlType::Bool => Self::Simple(IdlTypeSimple::Bool),
            anchor_idl::IdlType::U8 => Self::Simple(IdlTypeSimple::U8),
            anchor_idl::IdlType::I8 => Self::Simple(IdlTypeSimple::I8),
            anchor_idl::IdlType::U16 => Self::Simple(IdlTypeSimple::U16),
            anchor_idl::IdlType::I16 => Self::Simple(IdlTypeSimple::I16),
            anchor_idl::IdlType::U32 => Self::Simple(IdlTypeSimple::U32),
            anchor_idl::IdlType::I32 => Self::Simple(IdlTypeSimple::I32),
            anchor_idl::IdlType::F32 => Self::Simple(IdlTypeSimple::F32),
            anchor_idl::IdlType::U64 => Self::Simple(IdlTypeSimple::U64),
            anchor_idl::IdlType::I64 => Self::Simple(IdlTypeSimple::I64),
            anchor_idl::IdlType::F64 => Self::Simple(IdlTypeSimple::F64),
            anchor_idl::IdlType::U128 => Self::Simple(IdlTypeSimple::U128),
            anchor_idl::IdlType::I128 => Self::Simple(IdlTypeSimple::I128),
            anchor_idl::IdlType::Bytes => Self::Simple(IdlTypeSimple::Bytes),
            anchor_idl::IdlType::String => Self::Simple(IdlTypeSimple::String),
            anchor_idl::IdlType::PublicKey => Self::Simple(IdlTypeSimple::PublicKey),
            anchor_idl::IdlType::Defined(d) => {
                Self::Compound(IdlTypeCompound::Defined(IdlTypeDefined(d)))
            }
            anchor_idl::IdlType::Option(o) => Self::Compound(IdlTypeCompound::Option(
                IdlTypeOption(Box::new(IdlType::from(*o))),
            )),
            anchor_idl::IdlType::Vec(v) => Self::Compound(IdlTypeCompound::Vec(IdlTypeVec(
                Box::new(IdlType::from(*v)),
            ))),
            anchor_idl::IdlType::Array(a, size) => Self::Compound(IdlTypeCompound::Array(
                IdlTypeArray(Box::new(IdlType::from(*a)), size),
            )),
        }
    }
}

impl From<IdlType> for anchor_idl::IdlType {
    fn from(t: IdlType) -> Self {
        match t {
            IdlType::Simple(s) => Self::from(s),
            IdlType::Compound(c) => Self::from(c),
        }
    }
}

#[pymodule]
fn anchorpy_core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IdlTypeSimple>()?;
    m.add_class::<IdlTypeDefined>()?;
    m.add_class::<IdlTypeOption>()?;
    m.add_class::<IdlTypeVec>()?;
    m.add_class::<IdlTypeVec>()?;
    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let compound_members = vec![
        IdlTypeDefined::type_object(py),
        IdlTypeOption::type_object(py),
        IdlTypeVec::type_object(py),
        IdlTypeArray::type_object(py),
    ];
    m.add(
        "IdlTypeCompound",
        union.get_item(PyTuple::new(py, compound_members.clone()))?,
    )?;
    let mut idl_type_members = vec![IdlTypeSimple::type_object(py)];
    idl_type_members.extend(compound_members);
    m.add(
        "IdlType",
        union.get_item(PyTuple::new(py, idl_type_members))?,
    )?;
    Ok(())
}
