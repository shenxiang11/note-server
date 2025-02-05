-- Create User Table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    serial_number TEXT UNIQUE NOT NULL,
    fullname TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    avatar TEXT NOT NULL DEFAULT 'https://sns-avatar-qc.xhscdn.com/avatar/1040g2jo31bo9i342ge0040qk11mevsabtfhes5g',
    bio TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create Note Types
CREATE TYPE note_type AS ENUM ('normal', 'video');

-- Create Note Status
CREATE TYPE note_status AS ENUM ('draft', 'published');

-- Create Note Table
CREATE TABLE IF NOT EXISTS notes (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    status note_status NOT NULL DEFAULT 'draft',
    title TEXT NOT NULL,
    content TEXT,
    images TEXT[],
    video TEXT,
    type note_type NOT NULL DEFAULT 'normal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create Published Note Status
CREATE TYPE published_note_status AS ENUM ('published', 'hidden');

-- Create Published Note Table
CREATE TABLE IF NOT EXISTS published_notes (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    status published_note_status NOT NULL DEFAULT 'published',
    title TEXT NOT NULL,
    content TEXT,
    images TEXT[],
    video TEXT,
    type note_type NOT NULL DEFAULT 'normal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create Counter Biz Types
CREATE TYPE count_biz AS ENUM ('note_read', 'note_like', 'note_collect', 'note_comment');

-- Create Counter Table
CREATE TABLE IF NOT EXISTS counters (
    id BIGSERIAL PRIMARY KEY,
    biz count_biz NOT NULL,
    biz_id BIGINT NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (biz, biz_id)
);

-- Create History Biz Types
CREATE TYPE history_biz AS ENUM ('note');

-- Create User Read History Table
CREATE TABLE IF NOT EXISTS user_histories (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    biz history_biz NOT NULL,
    biz_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, biz, biz_id)
);

-- Create User Like Biz Types
CREATE TYPE user_likes_biz AS ENUM ('note', 'comment');

-- Create User Like Table
CREATE TABLE IF NOT EXISTS user_likes (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    biz user_likes_biz NOT NULL,
    biz_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    UNIQUE (user_id, biz, biz_id)
);

-- Create Comment Biz Types
CREATE TYPE comment_biz AS ENUM ('note', 'comment');

-- Create Comment Table
CREATE TABLE IF NOT EXISTS comments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    biz comment_biz NOT NULL,
    biz_id BIGINT NOT NULL,
    root_id BIGINT,
    parent_id BIGINT,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

-- Test Data
INSERT INTO "public"."users" ("id", "serial_number", "fullname", "email", "password_hash", "avatar", "bio", "created_at") VALUES
    (1, '20250114230001', '小黄人na7', '863461783@qq.com', '$argon2id$v=19$m=19456,t=2,p=1$Ogz47a+7nKodmkn4Hrjcag$S4rs/nyGR4108FuV4xMBkX4AXxaGi5ILp07FO8LV+NI', 'https://sns-avatar-qc.xhscdn.com/avatar/1040g2jo31bo9i342ge0040qk11mevsabtfhes5g', NULL, '2025-01-14 15:03:05.751299+00');
INSERT INTO "public"."users" ("id", "serial_number", "fullname", "email", "password_hash", "avatar", "bio", "created_at") VALUES
    (2, '20250202190002', '朱可人Coralie', '108182470@xhs.com', '$argon2id$v=19$m=19456,t=2,p=1$oHTBmwVEM01BSzbu/RRJIQ$Bmt2rEcIvnit1dwkiXDslb6G8mgRtu5FxVnYKdJ+jg0', 'https://sns-avatar-qc.xhscdn.com/avatar/1040g2jo31ati2qro6q0048f26jg653ipnnnjn80?imageView2/2/w/540/format/webp|imageMogr2/strip2', NULL, '2025-02-02 11:48:29.071396+00');

INSERT INTO "public"."notes" ("id", "user_id", "status", "title", "content", "images", "video", "type", "created_at", "updated_at") VALUES
    (1, 2, 'published', '长发留念', NULL, '{https://sns-webpic-qc.xhscdn.com/202502022142/987ea90496451c3ddb31e57ea92438f7/1040g2sg319b6ucds5o0048f26jg653ip3p7hs98!nd_dft_wlteh_webp_3,https://sns-webpic-qc.xhscdn.com/202502022142/286de2e9f4db2c536f725b530df9399c/1040g2sg319b6ucds5o0g48f26jg653ipa7v6c88!nd_dft_wlteh_webp_3}', NULL, 'normal', '2025-02-02 13:43:26.777+00', '2025-02-02 13:43:26.777+00');
INSERT INTO "public"."published_notes" ("id", "user_id", "status", "title", "content", "images", "video", "type", "created_at", "updated_at") VALUES
    (1, 2, 'published', '长发留念', NULL, '{https://sns-webpic-qc.xhscdn.com/202502022142/987ea90496451c3ddb31e57ea92438f7/1040g2sg319b6ucds5o0048f26jg653ip3p7hs98!nd_dft_wlteh_webp_3,https://sns-webpic-qc.xhscdn.com/202502022142/286de2e9f4db2c536f725b530df9399c/1040g2sg319b6ucds5o0g48f26jg653ipa7v6c88!nd_dft_wlteh_webp_3}', NULL, 'normal', '2025-02-02 13:43:26.842052+00', '2025-02-02 13:43:26.842052+00');
