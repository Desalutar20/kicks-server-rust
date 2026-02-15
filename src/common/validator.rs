#[macro_export]
macro_rules! validate_and_parse {
    ( $( $var:ident => $expr:expr ),* $(,)? ) => {{
        let mut errors = std::collections::HashMap::new();

        $(
            let $var: Option<_> = match $expr {
                Ok(val) => Some(val),
                Err(err) => {
                    if let $crate::Error::DomainValidationError(field_errors) = err {
                        errors.insert(stringify!($var).to_string(), field_errors);
                        None
                    } else {
                        return Err(err);
                    }
                }
            };
        )*

        if !errors.is_empty() {
            return Err($crate::Error::ValidationErrors(errors));
        }

        ( $( $var.unwrap() ),* )
    }};
}
