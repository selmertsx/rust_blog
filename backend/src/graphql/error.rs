use juniper::{graphql_value, FieldError, IntoFieldError};
use Error::*;

/// GraphQL Responseで返却するエラー一覧.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    // E1xx: 共通系ユーザーエラー
    NotFound,
    // E2xx: 個別系ユーザーエラー
    // E9xx: 共通系サーバーエラー
    Unexpected,
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            NotFound => FieldError::new("お探しのデータは見つかりませんでした。", graphql_value!({ "code": "E100" })),
            Unexpected => FieldError::new("想定外のエラーが発生しました。運営にて対応を行いますので、しばらくお待ちください。", graphql_value!({ "code": "E901" })),
        }
    }
}