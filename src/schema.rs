table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    files (hash) {
        hash -> Text,
        title -> Text,
        url -> Text,
        file_type -> FileTypeMapping,
    }
}
