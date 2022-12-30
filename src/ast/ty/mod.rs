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
