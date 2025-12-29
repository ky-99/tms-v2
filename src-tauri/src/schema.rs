// @generated automatically by Diesel CLI.

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        color -> Nullable<Text>,
        usage_count -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    task_queue (task_id) {
        task_id -> Text,
        position -> Integer,
        added_at -> Text,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
        parent_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(task_queue -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(tags, task_queue, task_tags, tasks,);
