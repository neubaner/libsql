#![allow(clippy::missing_safety_doc)]

#[derive(Debug)]
pub enum ValueType {
    Integer = 1,
    Real,
    Text,
    Blob,
    Null,
}

impl ValueType {
    pub fn from(val_type: i32) -> ValueType {
        match val_type as u32 {
            crate::ffi::SQLITE_INTEGER => ValueType::Integer,
            crate::ffi::SQLITE_FLOAT => ValueType::Real,
            crate::ffi::SQLITE_BLOB => ValueType::Blob,
            crate::ffi::SQLITE_TEXT => ValueType::Text,
            crate::ffi::SQLITE_NULL => ValueType::Null,
            _ => panic!("value type not supported"),
        }
    }

    pub fn from_str(s: &str) -> Option<ValueType> {
        match s {
            "TEXT" => Some(ValueType::Text),
            "INTEGER" => Some(ValueType::Integer),
            "BLOB" => Some(ValueType::Blob),
            "NULL" => Some(ValueType::Null),
            "REAL" => Some(ValueType::Real),
            _ => None,
        }
    }
}

pub struct Value {
    pub raw_value: *mut crate::ffi::sqlite3_value,
}

/* Reference from docs:
sqlite3_value_blob	→	BLOB value
sqlite3_value_double	→	REAL value
sqlite3_value_int	→	32-bit INTEGER value
sqlite3_value_int64	→	64-bit INTEGER value
sqlite3_value_pointer	→	Pointer value
sqlite3_value_text	→	UTF-8 TEXT value
sqlite3_value_text16	→	UTF-16 TEXT value in the native byteorder
sqlite3_value_text16be	→	UTF-16be TEXT value
sqlite3_value_text16le	→	UTF-16le TEXT value

sqlite3_value_bytes	→	Size of a BLOB or a UTF-8 TEXT in bytes
sqlite3_value_bytes16  	→  	Size of UTF-16 TEXT in bytes
sqlite3_value_type	→	Default datatype of the value
sqlite3_value_numeric_type  	→  	Best numeric datatype of the value
sqlite3_value_nochange  	→  	True if the column is unchanged in an UPDATE against a virtual table.
sqlite3_value_frombind  	→  	True if value originated from a bound parameter
*/

impl Value {
    pub fn value_type(&self) -> ValueType {
        let raw_type = unsafe { crate::ffi::sqlite3_value_type(self.raw_value) };
        ValueType::from(raw_type)
    }

    pub fn int(&self) -> i32 {
        unsafe { crate::ffi::sqlite3_value_int(self.raw_value) }
    }

    pub fn text(&self) -> *const u8 {
        unsafe { crate::ffi::sqlite3_value_text(self.raw_value) }
    }

    pub fn bytes(&self) -> i32 {
        unsafe { crate::ffi::sqlite3_value_bytes(self.raw_value) }
    }

    pub fn bytes16(&self) -> i32 {
        unsafe { crate::ffi::sqlite3_value_bytes16(self.raw_value) }
    }

    pub fn double(&self) -> f64 {
        unsafe { crate::ffi::sqlite3_value_double(self.raw_value) }
    }

    pub fn int64(&self) -> i64 {
        unsafe { crate::ffi::sqlite3_value_int64(self.raw_value) }
    }

    pub fn pointer(&self) -> *mut std::ffi::c_void {
        unsafe { crate::ffi::sqlite3_value_pointer(self.raw_value, std::ptr::null()) }
    }

    pub fn numeric_type(&self) -> i32 {
        unsafe { crate::ffi::sqlite3_value_numeric_type(self.raw_value) }
    }

    pub fn nochange(&self) -> bool {
        unsafe { crate::ffi::sqlite3_value_nochange(self.raw_value) != 0 }
    }

    pub fn frombind(&self) -> bool {
        unsafe { crate::ffi::sqlite3_value_frombind(self.raw_value) != 0 }
    }

    pub fn blob(&self) -> *const std::ffi::c_void {
        unsafe { crate::ffi::sqlite3_value_blob(self.raw_value) }
    }
}
