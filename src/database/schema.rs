table! {
    lang (id) {
        id -> Integer,
        user_id -> Integer,
        en -> Text,
        ja -> Nullable<Text>,
        ko -> Nullable<Text>,
        sk -> Nullable<Text>,
        cs -> Nullable<Text>,
        fr -> Nullable<Text>,
        es -> Nullable<Text>,
        pt -> Nullable<Text>,
        not_trans -> Integer,
        descripe -> Nullable<Text>,
        label -> Nullable<Text>,
        file_name -> Nullable<Text>,
        mode_name -> Nullable<Text>,
        project_id -> Integer,
        new_user_id -> Nullable<Integer>,
        new_en -> Nullable<Text>,
        new_ja -> Nullable<Text>,
        new_ko -> Nullable<Text>,
        new_sk -> Nullable<Text>,
        new_cs -> Nullable<Text>,
        new_fr -> Nullable<Text>,
        new_es -> Nullable<Text>,
        new_pt -> Nullable<Text>,
        new_not_trans -> Nullable<Integer>,
        new_descripe -> Nullable<Text>,
        new_label -> Nullable<Text>,
        new_file_name -> Nullable<Text>,
        new_mode_name -> Nullable<Text>,
        new_project_id -> Nullable<Integer>,
        status -> Integer,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

table! {
    project (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tem_lang (id) {
        id -> Nullable<Integer>,
        new_en -> Nullable<Text>,
        new_ja -> Nullable<Text>,
        new_ko -> Nullable<Text>,
        new_sk -> Nullable<Text>,
        new_cs -> Nullable<Text>,
        new_fr -> Nullable<Text>,
        new_es -> Nullable<Text>,
        new_pt -> Nullable<Text>,
        new_not_trans -> Nullable<Integer>,
        new_descripe -> Nullable<Text>,
        new_label -> Nullable<Text>,
        new_file_name -> Nullable<Text>,
        new_mode_name -> Nullable<Text>,
        new_project_id -> Nullable<Integer>,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        mail -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    lang,
    project,
    tem_lang,
    user,
);
