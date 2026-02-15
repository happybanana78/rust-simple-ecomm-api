CREATE TABLE product_review_attachments
(
    id                BIGSERIAL PRIMARY KEY,
    product_review_id BIGINT      NOT NULL,
    url               TEXT        NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_product_images_product_review_id
        FOREIGN KEY (product_review_id)
            REFERENCES product_reviews (id)
            ON DELETE CASCADE
);
