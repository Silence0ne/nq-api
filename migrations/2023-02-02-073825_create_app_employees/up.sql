-- Your SQL goes here
CREATE TABLE app_employees (
    id serial NOT NULL,
    org_account_id serial NOT NULL,
    employee_account_id  serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_employees_id PRIMARY KEY (id),
    UNIQUE(org_account_id, employee_account_id)
);