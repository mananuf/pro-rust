#[macro_export]
macro_rules! make_struct {
    // TODO: improve to make it create a full struct
    ($struct_name: ident; $field: ident, $type: ty) => {
        #[derive(Debug)]
        struct $struct_name {
            $field: $type
        }

        impl $struct_name {
            fn new($field: $type) -> Self {
                Self { $field }
            }
        }
    };
}