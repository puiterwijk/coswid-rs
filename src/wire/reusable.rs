macro_rules! set_if_empty {
    ($map_name:ident, $variable_name:ident) => {
        if $variable_name.is_some() {
            return Err(de::Error::duplicate_field(stringify!($variable_name)));
        }
        $variable_name = Some($map_name.next_value()?);
    }
}

macro_rules! require_field {
    ($field_name:ident) => {
        $field_name.ok_or_else(|| de::Error::missing_field(stringify!(field_name))
    }
}
