pub fn warn<T>(_:T) {}

macro_rules! define_macro {
    ($d:tt $lower:ident $upper:ident) => {
        #[macro_export] macro_rules! $upper  {
            ($d arg:tt) => { $crate:: $lower($d arg) }
        }
    }
}

define_macro!{$ warn  WARNING}
