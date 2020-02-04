CREATE TABLE tags (
    label varchar(16) PRIMARY KEY
)

CREATE TABLE products (
    id serial PRIMARY KEY
    nickname varchar(32) not null default 'noname'
)

CREATE TABLE tag_products (
    tag_label varchar(16) REFERENCES tags(label)
    product_id integer REFERENCES products(id)
)

CREATE TABLE receipts (
    id serial PRIMARY KEY
    product integer REFERENCES products(id)
    process_date date not null default now()
    price INTEGER not null default 0
)