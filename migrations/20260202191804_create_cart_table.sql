CREATE TABLE cart (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    user_hash_id BIGINT,
    total DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_cart_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE,

    CONSTRAINT fk_cart_user_hash
        FOREIGN KEY (user_hash_id)
            REFERENCES user_hashes(id)
            ON DELETE CASCADE
);
