// @generated automatically by Diesel CLI.
use diesel::associations::HasTable;

diesel::table! {
    country (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 3]
        iso3 -> Nullable<Bpchar>,
        #[max_length = 3]
        numeric_code -> Nullable<Bpchar>,
        #[max_length = 2]
        iso2 -> Nullable<Bpchar>,
        #[max_length = 255]
        phonecode -> Nullable<Varchar>,
        #[max_length = 255]
        capital -> Nullable<Varchar>,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        #[max_length = 255]
        currency_name -> Nullable<Varchar>,
        #[max_length = 255]
        currency_symbol -> Nullable<Varchar>,
        #[max_length = 255]
        tld -> Nullable<Varchar>,
        #[max_length = 255]
        native -> Nullable<Varchar>,
        #[max_length = 255]
        region -> Nullable<Varchar>,
        region_id -> Nullable<Int4>,
        #[max_length = 255]
        subregion -> Nullable<Varchar>,
        subregion_id -> Nullable<Int4>,
        #[max_length = 255]
        nationality -> Nullable<Varchar>,
        timezones -> Nullable<Text>,
        translations -> Nullable<Text>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        #[max_length = 191]
        emoji -> Nullable<Varchar>,
        #[max_length = 191]
        emoji_u -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        wiki_data_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    jwt_token (id) {
        id -> Uuid,
        token -> Text,
        active -> Bool,
    }
}

diesel::table! {
    language (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    server (id) {
        id -> Uuid,
        name -> Text,
        country_id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        name -> Text,
        password_hash -> Text,
        account_name -> Text,
        language_id -> Int4,
        password_salt -> Bytea,
    }
}

diesel::joinable!(server -> country (country_id));
diesel::joinable!(user -> country (language_id));

diesel::allow_tables_to_appear_in_same_query!(
    country,
    jwt_token,
    language,
    server,
    user,
);
