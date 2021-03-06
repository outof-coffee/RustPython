use crate::obj::objbytearray::{PyByteArray, PyByteArrayRef};
use crate::obj::objbytes::{PyBytes, PyBytesRef};
use crate::pyobject::PyObjectRef;
use crate::pyobject::{PyResult, TryFromObject, TypeProtocol};
use crate::vm::VirtualMachine;

pub enum PyBytesLike {
    Bytes(PyBytesRef),
    Bytearray(PyByteArrayRef),
}

impl TryFromObject for PyBytesLike {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        match_class!(match obj {
            b @ PyBytes => Ok(PyBytesLike::Bytes(b)),
            b @ PyByteArray => Ok(PyBytesLike::Bytearray(b)),
            obj => Err(vm.new_type_error(format!(
                "a bytes-like object is required, not {}",
                obj.class()
            ))),
        })
    }
}

impl PyBytesLike {
    pub fn to_cow(&self) -> std::borrow::Cow<[u8]> {
        match self {
            PyBytesLike::Bytes(b) => b.get_value().into(),
            PyBytesLike::Bytearray(b) => b.borrow_value().elements.clone().into(),
        }
    }

    #[inline]
    pub fn with_ref<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        match self {
            PyBytesLike::Bytes(b) => f(b.get_value()),
            PyBytesLike::Bytearray(b) => f(&b.borrow_value().elements),
        }
    }
}
