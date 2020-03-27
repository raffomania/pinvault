use super::schema::files;

#[derive(Queryable, Insertable)]
#[table_name="files"]
pub struct File {
    pub hash: String,
    pub url: String,
    pub title: String,
}
