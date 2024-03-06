// @generated automatically by Diesel CLI.

diesel::table! {
    urls (short_url) {
        #[max_length = 255]
        short_url -> Varchar,
        #[max_length = 255]
        long_url -> Varchar,
    }
}
