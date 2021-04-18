table! {
    file_list (id) {
        id -> Int4,
        file_name -> Varchar,
        md5 -> Text,
        commit_message -> Text,
        version_number -> Int4,
        user_uploader -> Varchar,
        upload_time -> Timestamp,
    }
}
