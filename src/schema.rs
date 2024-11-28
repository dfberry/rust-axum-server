// @generated automatically by Diesel CLI.

diesel::table! {
    osb_user (id) {
        id -> Text,
        github_id -> Text,
        username -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    osb_user_custom_config (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamptz
    }
}

diesel::joinable!(osb_user_custom_config -> osb_user (id));

diesel::allow_tables_to_appear_in_same_query!(
    osb_user,
    osb_user_custom_config,
);
