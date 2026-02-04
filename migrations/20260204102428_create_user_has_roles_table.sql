CREATE TABLE user_has_roles
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,

    CONSTRAINT fk_user_has_roles_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_user_has_roles_role_id
        FOREIGN KEY (role_id)
            REFERENCES roles (id)
            ON DELETE CASCADE
);
