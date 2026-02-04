CREATE TABLE personal_access_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    scopes JSONB NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT fk_personal_access_tokens_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
