use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sqlite::Sqlite;
use diesel::*;
use std::io::Write;

#[derive(SqlType)]
pub struct FileTypeMapping;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "FileTypeMapping"]
pub enum FileType {
    Video,
    Audio,
    Text,
    Image,
}

impl ToSql<FileTypeMapping, Sqlite> for FileType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        match *self {
            FileType::Video => out.write_all(b"video")?,
            FileType::Audio => out.write_all(b"audio")?,
            FileType::Text => out.write_all(b"text")?,
            FileType::Image => out.write_all(b"image")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FileTypeMapping, Sqlite> for FileType {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let string = <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)?;
        match string.as_str() {
            "video" => Ok(FileType::Video),
            "audio" => Ok(FileType::Audio),
            "image" => Ok(FileType::Image),
            "text" => Ok(FileType::Text),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl diesel::sql_types::HasSqlType<FileTypeMapping> for Sqlite {
    fn metadata(_lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
        diesel::sqlite::SqliteType::Text
    }
}

// see if we need this later
// impl FromSqlRow<FileType, Sqlite> for #enum_ty {
//     fn build_from_row<T: Row<Sqlite>>(row: &mut T) -> deserialize::Result<Self> {
//         FromSql::<#diesel_mapping, Sqlite>::from_sql(row.take())
//     }
// }
