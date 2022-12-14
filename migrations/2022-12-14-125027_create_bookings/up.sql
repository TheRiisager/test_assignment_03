CREATE TABLE bookings
(
    id           serial4      NOT NULL,
    customer_id  int4 NULL REFERENCES customers(id),
    booking_date date         NOT NULL,
    employee_id  int4 NULL REFERENCES public.employees(id),
    end_time     varchar(255) NOT NULL,
    start_time   varchar(255) NOT NULL,
    CONSTRAINT bookings_pkey PRIMARY KEY (id)
);