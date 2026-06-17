-- Migration: initial schema
-- Derived from domain model (domain-model.md)

-- 1. categories
CREATE TABLE categories (
    id   UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT        NOT NULL UNIQUE CHECK (name <> '')
);

INSERT INTO categories (id, name) VALUES
    (gen_random_uuid(), '書籍'),
    (gen_random_uuid(), 'ガジェット'),
    (gen_random_uuid(), 'ファッション'),
    (gen_random_uuid(), 'その他');

-- 2. wish_items
CREATE TYPE wish_item_status AS ENUM (
    'Inbox',
    'NextToBuy',
    'OnHold',
    'Archived',
    'Purchased'
);

CREATE TABLE wish_items (
    id          UUID              PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT              NOT NULL CHECK (name <> ''),
    price       BIGINT            NOT NULL CHECK (price >= 0),
    category_id UUID              NOT NULL REFERENCES categories(id),
    status      wish_item_status  NOT NULL DEFAULT 'Inbox',
    memo        TEXT              NOT NULL DEFAULT '',
    added_at    TIMESTAMPTZ       NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ       NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER wish_items_updated_at
    BEFORE UPDATE ON wish_items
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE INDEX idx_wish_items_status ON wish_items(status);
CREATE INDEX idx_wish_items_category_id ON wish_items(category_id);

-- 3. budgets
CREATE TABLE budgets (
    id        UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    year      SMALLINT    NOT NULL CHECK (year >= 2000),
    month     SMALLINT    NOT NULL CHECK (month BETWEEN 1 AND 12),
    amount    BIGINT      NOT NULL CHECK (amount > 0),
    balance   BIGINT      NOT NULL,
    set_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT budgets_unique_year_month UNIQUE (year, month)
);

-- 4. purchase_records
CREATE TABLE purchase_records (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    budget_id     UUID        NOT NULL REFERENCES budgets(id),
    wish_item_id  UUID        NOT NULL REFERENCES wish_items(id),
    actual_price  BIGINT      NOT NULL CHECK (actual_price >= 0),
    memo          TEXT        NOT NULL DEFAULT '',
    purchased_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_purchase_records_budget_id ON purchase_records(budget_id);
CREATE INDEX idx_purchase_records_wish_item_id ON purchase_records(wish_item_id);
