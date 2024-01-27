INSERT INTO "User" (id, username, email, name, surname, bio, profile_picture, password_hash, password_salt, created_at, edited_at)

VALUES
(
 8,
 'hafo',
 'h@h.com',
 'Hafan',
 'Hafski',
 'Som pes',
 '/home/hafo/profile_pic',
 'jfkldsajfo1ijroeowjfowef',
 '3dwqf',
 '2023-12-28 18:54:23+01',
 '2023-12-28 11:17:35+01'
),
(
    9,
    'pes',
    'pe@pe.com',
    'Hafan',
    'Hafski',
    'Som pes',
    '/home/hafo/profile_pic',
    'jfkldsajfo1ijroeowjfowef',
    '3dwqf',
    '2023-12-28 18:54:23+01',
    '2023-12-28 11:17:35+01'
);

ALTER SEQUENCE "User_id_seq" RESTART WITH 10;