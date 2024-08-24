// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        github_user -> Varchar,
    }
}

diesel::table! {
    watches (id) {
        id -> Text,
        github_user_id -> Text,
        org_repo_name -> Text,
        #[max_length = 30]
        watch_type -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(watches -> users (github_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    watches,
);
