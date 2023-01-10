use std::fmt;

use crate::wasm::WasmTy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    TyInt32,
    TyInt64,
    TyFloat32,
    TyFloat64,
    TyBool,
    Fn {
        params: Vec<Ty>,
        ret: Option<Box<Ty>>,
    },
    Void,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ty_string(&self))
    }
}

impl Ty {
    pub fn to_wasm_ty(&self) -> WasmTy {
        match self {
            Self::TyInt32 => WasmTy::Int32,
            Self::TyInt64 => WasmTy::Int64,
            Self::TyFloat32 => WasmTy::Float32,
            Self::TyFloat64 => WasmTy::Float64,
            _ => panic!("`{}` cannot convert to wasm type", self),
        }
    }
}

pub fn ty_string(ty: &Ty) -> String {
    match &ty {
        Ty::TyInt64 => "i64".to_string(),
        Ty::TyInt32 => "i32".to_string(),
        Ty::TyFloat64 => "f64".to_string(),
        Ty::TyFloat32 => "f32".to_string(),
        Ty::TyBool => "bool".to_string(),
        Ty::Void => "void".to_string(),
        Ty::Fn { params, ret } => {
            format!(
                "({}): {}",
                params.iter().map(ty_string).collect::<Vec<_>>().join(", "),
                match ret {
                    Some(ty) => ty_string(ty),
                    None => ty_string(&Ty::Void),
                }
            )
        }
    }
}
