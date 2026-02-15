CREATE TABLE product_reviews
(
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT,
    product_id      BIGINT      NOT NULL,
    title           TEXT        NOT NULL,
    content         TEXT        NOT NULL,
    rating          INT         NOT NULL,
    approval_status VARCHAR     NOT NULL,
    deleted_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_product_reviews_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_product_reviews_product_id
        FOREIGN KEY (product_id)
            REFERENCES products (id)
            ON DELETE CASCADE
);
