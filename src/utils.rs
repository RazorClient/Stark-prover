#[macro_export]
macro_rules! poly {
    ($($val:expr),* $(,)?) => {
        Polynomial::new(vec![$($val.into(),)*])
    }
}

// let p = poly![1, 3, 5]; 
#[macro_export]
macro_rules! fe {
    ($modulus:expr, $value:expr) => {
        FieldElement::<$modulus>::new($value)
    };
}
// let a = fe!(7, 3);
#[macro_export]
macro_rules! field {
    ($name:ident, $modulus:expr) => {
        type $name = FieldElement<$modulus>;
    };
}
// field!(Field7, 7);
// let b = Field7::new(5);
