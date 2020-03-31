use super::schema::files;
use super::sql_types::FileType;

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[table_name = "files"]
pub struct File {
    pub hash: String,
    pub title: String,
    pub url: String,
    pub file_type: FileType,
}
