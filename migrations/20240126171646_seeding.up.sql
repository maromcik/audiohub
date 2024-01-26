INSERT INTO "Genre" (id, name, color) VALUES (1,'Biography', '#9ACD32') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (2,'SciFi', '#6495ED') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (3,'Mystery', '#483D8B') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (4,'Fantasy', '#800080') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (5,'Crime', '#FF4500') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (6,'Horror', '#8B0000') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (7,'Thriller', '#FFD700') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (8,'Dystopian', '#2E8B57') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (9,'Magic Realism', '#FFA500') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (10,'Educational', '#87CEEB') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (11,'Romance', '#FF69B4') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (12,'Business and Economics', '#4169E1') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (13,'Kids', '#00BFFF') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (14,'Cooking', '#CD853F') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (15,'Fairy Tales', '#FF6347') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (16,'Novels', '#008080') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (17,'History', '#8B4513') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (18,'Adventure', '#228B22') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (19,'Sports', '#FF8C00') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (20,'Entertainment', '#FFD700') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (21,'Travel', '#32CD32') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (22,'Politics', '#800000') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (23,'Motorsport', '#FF0000') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (24,'Computers', '#00CED1') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (25,'Art', '#FFD700') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (26,'Fiction', '#9400D3') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (27,'Tragedy', '#FF5E00') ON CONFLICT DO NOTHING;
INSERT INTO "Genre" (id, name, color) VALUES (28,'Documentary', '#4000FF') ON CONFLICT DO NOTHING;


INSERT INTO "User" (id, username, email, name, surname, bio, profile_picture, password_hash, password_salt)
VALUES (1, 'charlie', 'c@c.com', 'Charles', 'Dickens', 'We forge the chains we wear in life.', '/media/examples/c.jpg', '$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw', 'xHr+sXyp5BtpPCIvIRrRvA')
ON CONFLICT DO NOTHING;
INSERT INTO "User" (id, username, email, name, surname, bio, profile_picture, password_hash, password_salt)
VALUES (2, 'sop', 's@s.com', 'Sophocles', 'of Greece', 'The only true wisdom is in knowing you know nothing.', '/media/examples/s.jpg', '$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw', 'xHr+sXyp5BtpPCIvIRrRvA')
ON CONFLICT DO NOTHING;
INSERT INTO "User" (id, username, email, name, surname, bio, profile_picture, password_hash, password_salt)
VALUES (3, 'archie', 'a@a.com', 'Arthur Conan', 'Doyle', 'I like detectives', '/media/examples/a.jpg', '$pbkdf2-sha256$i=600000,l=32$xHr+sXyp5BtpPCIvIRrRvA$yjmxouyWA7I4mHhTPutuHThixR0gz7nuhYTgFJYAOYw', 'xHr+sXyp5BtpPCIvIRrRvA')
ON CONFLICT DO NOTHING;

ALTER SEQUENCE "User_id_seq" RESTART WITH 4;

INSERT INTO "Audiobook" (id,
                         genre_id,
                         author_id,
                         name,
                         file_path,
                         length,
                         thumbnail,
                         description,
                         stream_count,
                         like_count)
VALUES (1,
        26,
        1,
        'Oliver Twist - Example',
        '/media/examples/ot.mp3',
        456.378,
        '/media/examples/ot.jpg',
        'Oliver Twist unromantically portrays the sordid lives of criminals and exposes the cruel treatment of the many orphans in London in the mid-19th century.',
        2,
        1)
ON CONFLICT DO NOTHING;

INSERT INTO "Audiobook" (id,
                         genre_id,
                         author_id,
                         name,
                         file_path,
                         length,
                         thumbnail,
                         description,
                         stream_count,
                         like_count)
VALUES (2,
        27,
        2,
        'Antigone',
        '/media/examples/antigone.mp3',
        4446.702,
        '/media/examples/antigone.jpg',
        'Antigone is an Athenian tragedy written by Sophocles in 441 BC and first performed at the Festival of Dionysus of the same year.',
        1,
        2)
ON CONFLICT DO NOTHING;

INSERT INTO "Audiobook" (id,
                         genre_id,
                         author_id,
                         name,
                         file_path,
                         length,
                         thumbnail,
                         description,
                         stream_count,
                         like_count)
VALUES (3,
        3,
        3,
        'Adventures of Sherlock Holmes - Example',
        '/media/examples/sh.mp3',
        1662.856,
        '/media/examples/sh.jpg',
        'A collection of twelve short stories featuring Conan Doyle''s legendary detective, originally published as single stories in Strand Magazine and subsequently collected into a single volume. There is not always a crime committed nor a culprit to find, and when there is, Holmes does not invariably get his man.',
        2,
        3)
ON CONFLICT DO NOTHING;

INSERT INTO "Audiobook" (id,
                         genre_id,
                         author_id,
                         name,
                         file_path,
                         length,
                         thumbnail,
                         description,
                         stream_count,
                         like_count)
VALUES (4,
        16,
        1,
        'Tale of Two Cities - Example',
        '/media/examples/totc.mp3',
        407.914,
        '/media/examples/totc.jpg',
        'A Tale of Two Cities is a historical novel published in 1859 by Charles Dickens, set in London and Paris before and during the French Revolution.',
        1,
        1)
ON CONFLICT DO NOTHING;

ALTER SEQUENCE "Audiobook_id_seq" RESTART WITH 5;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (1, 'Introduction', 2, 0)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (2, 'Conclusion', 2, 2540)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (3, 'A', 1, 10)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (4, 'B', 1, 60)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (5, 'C', 1, 250)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (6, 'd', 1, 300)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (7, 'Adventure 1 - Intro', 3, 0)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (8, 'Adventure 1 - Story', 3, 100)
ON CONFLICT DO NOTHING;

INSERT INTO "Chapter" (id, name, audiobook_id, position)
VALUES (9, 'Adventure 1 - Conclusion', 3, 1400)
ON CONFLICT DO NOTHING;

ALTER SEQUENCE "Chapter_id_seq" RESTART WITH 10;

INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (1, 3, 50) ON CONFLICT DO NOTHING;
INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (1, 2, 4446) ON CONFLICT DO NOTHING;
INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (2, 1, 456) ON CONFLICT DO NOTHING;
INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (2, 3, 457) ON CONFLICT DO NOTHING;
INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (3, 1, 200) ON CONFLICT DO NOTHING;
INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position) VALUES (3, 4, 407) ON CONFLICT DO NOTHING;


INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (1, 2) ON CONFLICT DO NOTHING;
INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (1, 3) ON CONFLICT DO NOTHING;
INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (1, 4) ON CONFLICT DO NOTHING;
INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (2, 3) ON CONFLICT DO NOTHING;
INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (2, 4) ON CONFLICT DO NOTHING;
INSERT INTO "Bookmark" (user_id, audiobook_id) VALUES (3, 4) ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (1, 1, 2, 4, 'I appreciate this book, but it feels kinda old these days.')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (2, 3, 2, 3, 'I think we got past such boring stories.')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (3, 2, 2, 5, 'You are mean guys.')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (4, 1, 1, 5, 'Absolute banger')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (5, 2, 1, 1, 'A very twisted story')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (6, 3, 1, 4, 'Great')
ON CONFLICT DO NOTHING;


INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (7, 2, 4, 1, 'I did not like either of the cities')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (8, 1, 4, 5, 'Great')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (9, 3, 4, 3, 'Nah')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (10, 2, 3, 1, 'Confusing')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (11, 1, 3, 3, 'Average')
ON CONFLICT DO NOTHING;

INSERT INTO "Rating" (id, user_id, audiobook_id, rating, review)
VALUES (12, 3, 3, 5, 'Thrilling')
ON CONFLICT DO NOTHING;

ALTER SEQUENCE "Rating_id_seq" RESTART WITH 13;