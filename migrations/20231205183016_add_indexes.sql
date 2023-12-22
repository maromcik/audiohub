-- FKs
CREATE INDEX IF NOT EXISTS "Audiobook_author_id_idx" ON "audiobooks".public."Audiobook" (author_id);
CREATE INDEX IF NOT EXISTS "Audiobook_genre_id_id_idx" ON "audiobooks".public."Audiobook" (genre_id);
CREATE INDEX IF NOT EXISTS "Audiobook_publisher_id_id_idx" ON "audiobooks".public."Audiobook" (publisher_id);
CREATE INDEX IF NOT EXISTS "Chapter_audiobook_id_idx" ON "audiobooks".public."Chapter" (audiobook_id);
CREATE INDEX IF NOT EXISTS "Rating_audiobook_id_idx" ON "audiobooks".public."Rating" (audiobook_id);
CREATE INDEX IF NOT EXISTS "Rating_userid_id_idx" ON "audiobooks".public."Rating" (user_id);

-- FKs on M-to-N
-- CREATE INDEX IF NOT EXISTS "Bookmark_user_id_idx" ON "audiobooks".public."Bookmark" (user_id);
-- CREATE INDEX IF NOT EXISTS "Bookmark_audiobook_id_idx" ON "audiobooks".public."Bookmark" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_user_id_idx" ON "audiobooks".public."Active_Audiobook" (user_id);
-- CREATE INDEX IF NOT EXISTS "Active_Audiobook_audiobook_id_idx" ON "audiobooks".public."Active_Audiobook" (audiobook_id);
--
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_author_id_idx" ON "audiobooks".public."Audiobook_Author" (author_id);
-- CREATE INDEX IF NOT EXISTS "Audiobook_Author_audiobook_id_idx" ON "audiobooks".public."Audiobook_Author" (audiobook_id);

