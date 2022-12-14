CREATE TABLE employees
(
    id         serial4 NOT NULL,
    birthdate  date NULL,
    first_name varchar(255) NULL,
    last_name  varchar(255) NULL,
    CONSTRAINT employees_pkey PRIMARY KEY (id)
);