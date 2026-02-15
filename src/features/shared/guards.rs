use crate::Error;

pub fn map_unique_violation(custom_error: Option<Error>) -> impl FnOnce(Error) -> Error {
    move |error| match error {
        Error::Database(err)
            if err
                .as_database_error()
                .is_some_and(|e| e.is_unique_violation()) =>
        {
            custom_error.unwrap_or(Error::Conflict("Record already exists".into()))
        }
        _ => error,
    }
}
