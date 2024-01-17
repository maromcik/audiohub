ALTER TABLE "Audiobook"
    DROP CONSTRAINT IF EXISTS "Audiobook_author_id_fkey",
    ADD CONSTRAINT "Audiobook_author_id_fkey"
        FOREIGN KEY (author_id)
            REFERENCES "User"(id);


ALTER TABLE "Audiobook"
    DROP CONSTRAINT IF EXISTS "Audiobook_genre_id_fkey",
    ADD CONSTRAINT "Audiobook_genre_id_fkey"
        FOREIGN KEY (genre_id)
            REFERENCES "Genre"(id);


ALTER TABLE "Chapter"
    DROP CONSTRAINT IF EXISTS "Chapter_audiobook_id_fkey",
    ADD CONSTRAINT "Chapter_audiobook_id_fkey"
        FOREIGN KEY (audiobook_id)
            REFERENCES "Audiobook"(id);

ALTER TABLE "Rating"
    DROP CONSTRAINT IF EXISTS "Rating_audiobook_id_fkey",
    ADD CONSTRAINT "Rating_audiobook_id_fkey"
        FOREIGN KEY (audiobook_id)
            REFERENCES "Audiobook"(id);

ALTER TABLE "Rating"
    DROP CONSTRAINT IF EXISTS "Rating_user_id_fkey",
    ADD CONSTRAINT "Rating_user_id_fkey"
        FOREIGN KEY (user_id)
            REFERENCES "User"(id);

ALTER TABLE "Bookmark"
    DROP CONSTRAINT IF EXISTS "Bookmark_user_id_fkey",
    ADD CONSTRAINT "Bookmark_user_id_fkey"
        FOREIGN KEY (user_id)
            REFERENCES "User"(id);

ALTER TABLE "Bookmark"
    DROP CONSTRAINT IF EXISTS "Bookmark_audiobook_id_fkey",
    ADD CONSTRAINT "Bookmark_audiobook_id_fkey"
        FOREIGN KEY (audiobook_id)
            REFERENCES "Audiobook"(id);

ALTER TABLE "Active_Audiobook"
    DROP CONSTRAINT IF EXISTS "Active_Audiobook_user_id_fkey",
    ADD CONSTRAINT "Active_Audiobook_user_id_fkey"
        FOREIGN KEY (user_id)
            REFERENCES "User"(id);

ALTER TABLE "Active_Audiobook"
    DROP CONSTRAINT IF EXISTS "Active_Audiobook_audiobook_id_fkey",
    ADD CONSTRAINT "Active_Audiobook_audiobook_id_fkey"
        FOREIGN KEY (audiobook_id)
            REFERENCES "Audiobook"(id);

ALTER TABLE "Active_Audiobook"
    DROP CONSTRAINT IF EXISTS "Active_Audiobook_playback_chapter_id_fkey",
    ADD CONSTRAINT "Active_Audiobook_playback_chapter_id_fkey"
        FOREIGN KEY (playback_chapter_id)
            REFERENCES "Chapter"(id);