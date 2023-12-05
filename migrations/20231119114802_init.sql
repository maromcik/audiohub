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

/* Post creator id needs to be indexed, as it is accessed often */
-- CREATE INDEX IF NOT EXISTS "PostCreatorId" ON "Post" (creator_id);
/* Post's created / deleted at need to be indexed */
-- CREATE INDEX IF NOT EXISTS "PostCreatedDeletedAt" ON "Post" (created_at DESC, deleted_at NULLS LAST);


CREATE TABLE IF NOT EXISTS "Publisher"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name         text        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);

/* Comment's creator needs to be indexed */
-- CREATE INDEX IF NOT EXISTS "CommentCreatorId" ON "Comment" (commenter_id);
/* Comment's post id needs to be indexed */
-- CREATE INDEX IF NOT EXISTS "CommentPostId" ON "Comment" (post_id);
/* Comment's created / deleted at need to be indexed */
-- CREATE INDEX IF NOT EXISTS "CommentCreatedDeletedAt" ON "Comment" (created_at DESC, deleted_at NULLS LAST);


CREATE TABLE IF NOT EXISTS "Genre"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name         text        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);

CREATE TABLE IF NOT EXISTS "Chapter"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name                text            NOT NULL,
    audiobook_id        bigserial       NOT NULL,
    length              interval        NOT NULL,
    sequential_number   int             NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);


CREATE TABLE IF NOT EXISTS "Author"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    name       text        NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz
);


CREATE TABLE IF NOT EXISTS "Audiobook"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    publisher_id        bigserial       NOT NULL,
    genre_id            bigserial       NOT NULL,
    audiobook_id        bigserial       NOT NULL,
    name                text            NOT NULL,
    price_dollars       decimal         NOT NULL,
    price_cents         decimal         NOT NULL,
    length              interval        NOT NULL,
    file_path           text            NOT NULL,
    stream_count        bigint          NOT NULL,
    overall_rating      smallint        NOT NULL,

    FOREIGN KEY (publisher_id)      REFERENCES "Publisher" (id),
    FOREIGN KEY (genre_id)          REFERENCES "Genre" (id),
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
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    user_id         bigserial        NOT NULL,
    audiobook_id    bigserial        NOT NULL,

    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);


CREATE TABLE IF NOT EXISTS "Audiobook_User"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    user_id                         bigserial        NOT NULL,
    audiobook_id                    bigserial        NOT NULL,
    playback_chapter_id             bigserial,
    playback_position_in_chapter    interval,

    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Chapter" (id)
);

CREATE TABLE IF NOT EXISTS "Audiobook_Author"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    author_id           bigserial        NOT NULL,
    audiobook_id        bigserial        NOT NULL,

    FOREIGN KEY (author_id)       REFERENCES "Author" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);