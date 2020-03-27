#[derive(Queryable)]
pub struct File {
    pub hash: String,
    pub url: String,
    pub title: String,
}
