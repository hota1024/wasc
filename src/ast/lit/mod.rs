pub mod LitIdent;
pub mod LitUnsignedInt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    LitUnsignedInt(LitUnsignedInt::LitUnsignedInt),
    LitIdent(LitIdent::LitIdent),
}
