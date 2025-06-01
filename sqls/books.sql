-- テーブルが存在する場合は削除 (開発時など、既存のテーブルをクリアしたい場合に便利)
-- 注意: 本番環境での運用には慎重な検討が必要です
DROP TABLE IF EXISTS book_authors;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;
-- authors テーブルの作成
CREATE TABLE authors (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- 著者ID (Primary Key)
    name_ VARCHAR(255) NOT NULL -- 著者名
);
-- books テーブルの作成
CREATE TABLE books (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    -- 書籍ID (Primary Key)
    title VARCHAR(255) NOT NULL,
    -- タイトル
    published_date DATE,
    -- 出版年月日
    isbn VARCHAR(255) UNIQUE,
    -- ISBN (国際標準図書番号) - UNIQUE制約を追加して一意性を保証
    thumbnail_url VARCHAR(255),
    -- サムネイルURL
    description_ TEXT -- 書籍の概要または説明
);
-- book_authors テーブルの作成 (中間テーブル)
CREATE TABLE book_authors (
    book_id VARCHAR(255) NOT NULL,
    -- 書籍ID
    author_id VARCHAR(255) NOT NULL,
    -- 著者ID
    PRIMARY KEY (book_id, author_id),
    -- 複合主キー
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    -- booksテーブルへの外部キー参照
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE -- authorsテーブルへの外部キー参照
);
