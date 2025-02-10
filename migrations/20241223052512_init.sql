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
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
