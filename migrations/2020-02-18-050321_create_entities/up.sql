-- Your SQL goes here
CREATE TABLE lang (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL,
    "user_id" integer NOT NULL, 
    "en" text NOT NULL, 
    "ja" text, 
    "ko" text, 
    "sk" text, 
    "cs" text, 
    "fr" text, 
    "es" text,
    "pt" text,
    "not_trans" integer NOT NULL DEFAULT (0), 
    "descripe" text, 
    "label" text, 
    "file_name" text, 
    "mode_name" text, 
    "project_id" integer NOT NULL, 
    "new_user_id" integer, 
    "new_en" text, 
    "new_ja" text, 
    "new_ko" text, 
    "new_sk" text, 
    "new_cs" text, 
    "new_fr" text, 
    "new_es" text, 
    "new_pt" text, 
    "new_not_trans" integer, 
    "new_descripe" text, 
    "new_label" text, 
    "new_file_name" text, 
    "new_mode_name" text, 
    "new_project_id" integer, 
    "status" integer NOT NULL DEFAULT (0), 
    "create_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "update_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE project (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "name" varchar NOT NULL
);
CREATE TABLE user (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "username" varchar NOT NULL, 
    "mail" varchar NOT NULL, 
    "password" varchar NOT NULL
);
CREATE TABLE tem_lang (
    "id" integer PRIMARY KEY,
    "new_en" text, 
    "new_ja" text, 
    "new_ko" text, 
    "new_sk" text, 
    "new_cs" text, 
    "new_fr" text, 
    "new_es" text, 
    "new_pt" text, 
    "new_not_trans" integer, 
    "new_descripe" text, 
    "new_label" text,
    "new_file_name" text, 
    "new_mode_name" text, 
    "new_project_id" integer
);