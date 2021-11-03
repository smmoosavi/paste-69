#[macro_export]
macro_rules! the_macro {
    ($result:ident, $field:ident) => {
        paste::paste! {
            struct $result {
                [<$field>]: String,
            }
        }
    };
}
