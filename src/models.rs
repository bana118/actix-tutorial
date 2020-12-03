use super::schema::memos;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Memo {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "memos"]
pub struct NewMemo {
    pub content: String,
}
