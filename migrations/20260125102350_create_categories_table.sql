CREATE TABLE categories
(
    id         BIGSERIAL PRIMARY KEY,
    name       TEXT        NOT NULL,
    slug       TEXT        NOT NULL UNIQUE,
    is_active  BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
