pub use varienum_macro::VariantsVec;

pub trait VariantsVec {
    fn variants() -> &'static [&'static str];
    fn variants_desc() -> &'static [(&'static str, &'static str)];
}

