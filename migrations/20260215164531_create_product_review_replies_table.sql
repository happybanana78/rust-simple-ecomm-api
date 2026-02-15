CREATE TABLE product_review_replies
(
    id                BIGSERIAL PRIMARY KEY,
    product_review_id BIGINT      NOT NULL,
    content           TEXT        NOT NULL,
    deleted_at        TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_product_reviews_product_review_id
        FOREIGN KEY (product_review_id)
            REFERENCES product_reviews (id)
            ON DELETE CASCADE
);
