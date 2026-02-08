CREATE TABLE products
(
    id           BIGSERIAL PRIMARY KEY,
    name         TEXT             NOT NULL,
    price        DOUBLE PRECISION NOT NULL,
    quantity     INTEGER          NOT NULL DEFAULT 0,
    configurable BOOLEAN          NOT NULL DEFAULT FALSE,
    is_active    BOOLEAN          NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ      NOT NULL DEFAULT now()
);
