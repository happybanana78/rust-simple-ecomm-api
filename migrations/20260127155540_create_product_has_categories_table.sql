CREATE TABLE product_has_categories
(
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL,
    product_id  BIGINT NOT NULL,

    CONSTRAINT fk_product_has_categories_category_id
        FOREIGN KEY (category_id)
            REFERENCES categories (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_product_has_categories_product_id
        FOREIGN KEY (product_id)
            REFERENCES products (id)
            ON DELETE CASCADE
);
