CREATE TABLE IF NOT EXISTS "User"
(
    id              bigserial PRIMARY KEY,
    ---------------------------------------------
    username        text UNIQUE NOT NULL,
    email           text UNIQUE NOT NULL,
    name            text        NOT NULL,
    surname         text        NOT NULL,
    bio             text        NOT NULL,
    profile_picture text        NOT NULL,
    password_hash   text        NOT NULL,
    password_salt   text        NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz
);


CREATE TABLE IF NOT EXISTS "Genre"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name         text        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);



CREATE TABLE IF NOT EXISTS "Audiobook"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    genre_id            bigserial       NOT NULL,
    author_id           bigserial       NOT NULL,
    name                text            NOT NULL,
    file_path           text            NOT NULL,
    thumbnail           text            NOT NULL,
    description         text            NOT NULL,
    stream_count        bigint          NOT NULL DEFAULT 0,
    like_count          bigint          NOT NULL DEFAULT 0,
    overall_rating      smallint        NOT NULL DEFAULT 0,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,

    FOREIGN KEY (genre_id)          REFERENCES "Genre" (id),
    FOREIGN KEY (author_id)      REFERENCES "User" (id)
);

CREATE TABLE IF NOT EXISTS "Chapter"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name                text            NOT NULL,
    audiobook_id        bigserial       NOT NULL,
    position            float8          NOT NULL DEFAULT 0,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,

    FOREIGN KEY (audiobook_id)      REFERENCES "Audiobook" (id)
);

CREATE TABLE IF NOT EXISTS "Rating"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    user_id         bigserial        NOT NULL,
    audiobook_id    bigserial        NOT NULL,
    rating          smallint    NOT NULL,
    review          text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz,

    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);

CREATE TABLE IF NOT EXISTS "Bookmark"
(
    user_id         bigserial        NOT NULL,
    audiobook_id    bigserial        NOT NULL,

    PRIMARY KEY (user_id, audiobook_id),
    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);


CREATE TABLE IF NOT EXISTS "Active_Audiobook"
(
    user_id                         bigserial        NOT NULL,
    audiobook_id                    bigserial        NOT NULL,
    playback_chapter_id             bigserial,
    playback_position_in_chapter    interval        DEFAULT make_interval(0,0,0,0,0,0,0),

    PRIMARY KEY (user_id, audiobook_id, playback_chapter_id),
    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Chapter" (id)
);

CREATE INDEX IF NOT EXISTS "Audiobook_author_id_idx" ON "Audiobook" (author_id);
CREATE INDEX IF NOT EXISTS "Audiobook_genre_id_id_idx" ON "Audiobook" (genre_id);
CREATE INDEX IF NOT EXISTS "Chapter_audiobook_id_idx" ON "Chapter" (audiobook_id);
CREATE INDEX IF NOT EXISTS "Rating_audiobook_id_idx" ON "Rating" (audiobook_id);
CREATE INDEX IF NOT EXISTS "Rating_userid_id_idx" ON "Rating" (user_id);

-- FKs on M-to-N
-- CREATE INDEX IF NOT EXISTS "Bookmark_user_id_idx" ON "audiobooks".public."Bookmark" (user_id);
-- CREATE INDEX IF NOT EXISTS "Bookmark_audiobook_id_idx" ON "audiobooks".public."Bookmark" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_user_id_idx" ON "audiobooks".public."Active_Audiobook" (user_id);
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_audiobook_id_idx" ON "audiobooks".public."Active_Audiobook" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_author_id_idx" ON "audiobooks".public."Audiobook_Author" (author_id);
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_audiobook_id_idx" ON "audiobooks".public."Audiobook_Author" (audiobook_id);

