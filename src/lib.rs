use std::{
    ffi::{c_char, c_int, c_void, CString},
    ptr::slice_from_raw_parts,
};

const sz: usize = 256;
type RAsmOpString = [c_char; sz];

#[repr(C)]
pub struct RAsmOp {
    size: c_int,
    payload: c_int,
    buf: RAsmOpString,
    buf_asm: RAsmOpString,
    buf_hex: RAsmOpString,
}

#[repr(C)]
pub struct RAsmPluginC {
    name: CString,
    license: CString,
    description: CString,
    arch: CString,
    bits: c_int,
    endian: c_int,
    dissassemble_function: Box<dyn Fn(*const c_void, *mut RAsmOp, *const u8, c_int) -> c_int>,
}

impl From<Box<dyn DissassemblyPlugin>> for RAsmPluginC {
    fn from(wrapper: Box<dyn DissassemblyPlugin>) -> Self {
        RAsmPluginC {
            name: CString::new(wrapper.name()).unwrap(),
            license: CString::new(wrapper.license()).unwrap(),
            description: CString::new(wrapper.description()).unwrap(),
            arch: CString::new(wrapper.arch()).unwrap(),
            bits: wrapper.bits().into(),
            endian: wrapper.endian().into(),
            dissassemble_function: Box::new(
                move |_rasm: *const c_void,
                      rasm_op: *mut RAsmOp,
                      buf: *const u8,
                      buf_len: c_int| unsafe {
                    let rust_buf = slice_from_raw_parts(buf, buf_len as usize);
                    *rasm_op = wrapper.dissassemble(rust_buf);
                    return (*rasm_op).size;
                },
            ),
        }
    }
}
// static int disassemble(RAsm *a, RAsmOp *op, const ut8 *buf, int len)

#[repr(C)]
pub enum Endianness {
    Big,
    Small,
}

impl Into<c_int> for Endianness {
    fn into(self) -> c_int {
        match self {
            Self::Big => 0,
            Self::Small => 1,
        }
    }
}
// pub struct RAsmPluginWrapper {
//     name: String,
//     license: String,
//     description: String,
//     arch: String,
//     bits: u8,
//     endian: Endianness,
//     // dissassemble_function: F,
// }

pub trait DissassemblyPlugin {
    // fn dissassemble(&self, buf: &'a [u8]) -> RAsmOp;
    fn dissassemble(&self, buf: *const [u8]) -> RAsmOp;

    fn name(&self) -> String;
    fn license(&self) -> String;
    fn description(&self) -> String;
    fn arch(&self) -> String;
    fn bits(&self) -> u8;
    fn endian(&self) -> Endianness;
}

#[cfg(test)]
mod tests {}
