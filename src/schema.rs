// @generated automatically by Diesel CLI.

diesel::table! {
    hodies (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        brand -> Nullable<Varchar>,
        #[max_length = 255]
        category -> Nullable<Varchar>,
    }
}
