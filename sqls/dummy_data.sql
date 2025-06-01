INSERT INTO books (
        id,
        title,
        published_date,
        isbn,
        thumbnail_url,
        description_
    )
VALUES (
        'aC8wEQAAQBAJ',
        'バックエンドエンジニアを目指す人のためのRust',
        '2024-10-25',
        '9784798186016',
        'http://books.google.com/books/content?id=aC8wEQAAQBAJ&printsec=frontcover&img=1&zoom=5&edge=curl&source=gbs_api',
        'Rustでプログラムを作りながら、課題を細分化し、解決する力を鍛える！'
    );
INSERT INTO authors (name_)
VALUES ('安東 一慈'),
    ('大西 諒'),
    ('徳永 裕介'),
    ('中村 謙弘'),
    ('山中 雄大');
INSERT INTO book_authors (book_id, author_id)
VALUES ('aC8wEQAAQBAJ', 1),
    ('aC8wEQAAQBAJ', 2),
    ('aC8wEQAAQBAJ', 3),
    ('aC8wEQAAQBAJ', 4),
    ('aC8wEQAAQBAJ', 5);
