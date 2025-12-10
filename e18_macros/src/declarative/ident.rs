#[macro_export]
macro_rules! make_variable {
    ($name: ident) => {
        let $name = "Macro Variable";
    };
}


// exercise
#[macro_export]
macro_rules! make_getter {
    ($field_name: ident, $FieldType: ty) => {
        pub fn $field_name (&self) -> $FieldType {
            &self.$field_name
        }
    };
}