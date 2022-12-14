CREATE TABLE customers
(
    id         serial4 NOT NULL,
    birthdate  date NULL,
    first_name varchar(255) NULL,
    last_name  varchar(255) NULL,
    mobile     varchar(255) NULL,
    CONSTRAINT customers_pkey PRIMARY KEY (id)
);