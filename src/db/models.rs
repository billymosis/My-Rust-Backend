use super::schema::file_list;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct FileList {
    pub id: i32,
    pub file_name: String,
    pub md5: String,
    pub commit_message: String,
    pub version_number: i32,
    pub user_uploader: String,
    pub upload_time: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "file_list"]
pub struct NewFile<'a> {
    pub file_name: &'a str,
    pub md5: &'a str,
    pub commit_message: &'a str,
    pub version_number: i32,
    pub user_uploader: &'a str,
}
