CREATE TABLE product_videos
(
    id         BIGSERIAL PRIMARY KEY,
    product_id BIGINT      NOT NULL,
    url        TEXT        NOT NULL,
    alt        TEXT        NOT NULL,
    is_main    BOOLEAN     NOT NULL DEFAULT false,
    sort       NUMERIC     NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_product_images_product_id
        FOREIGN KEY (product_id)
            REFERENCES products (id)
            ON DELETE CASCADE
);
