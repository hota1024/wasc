pub enum WasmTy {
    Int32,
    Int64,
    Float32,
    Float64,
}

impl std::fmt::Display for WasmTy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WasmTy::Int32 => "i32",
                WasmTy::Int64 => "i64",
                WasmTy::Float32 => "f32",
                WasmTy::Float64 => "f64",
            }
        )
    }
}
