CREATE TABLE cart_items
(
    id         BIGSERIAL PRIMARY KEY,
    cart_id    BIGINT           NOT NULL,
    product_id BIGINT           NOT NULL,
    price      DOUBLE PRECISION NOT NULL,
    quantity   INTEGER          NOT NULL,
    created_at TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_cart_items_cart
        FOREIGN KEY (cart_id)
            REFERENCES cart (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_cart_items_product
        FOREIGN KEY (product_id)
            REFERENCES products (id)
            ON DELETE CASCADE
);
