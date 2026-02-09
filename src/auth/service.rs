use crate::auth::dto::{AuthToken, LoginCommand, NewUser, PublicAuthToken, RegisterCommand};
use crate::auth::model::{AuthTokenModel, UserModel};
use crate::auth::repository::AuthRepository;
use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::roles::service::RoleService;
use argon2::password_hash::phc::PasswordHash;
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHasher};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AuthService {
    pub pool: PgPool,
    pub repository: AuthRepository,
    pub roles_service: RoleService,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            repository: AuthRepository::new(),
            roles_service: RoleService::new(pool),
        }
    }

    pub async fn register(&self, cmd: RegisterCommand) -> Result<UserModel, AppError> {
        let hashed_password = hash_password(cmd.password.as_str())?;

        let new_user = NewUser {
            username: cmd.username,
            email: cmd.email,
            hashed_password,
        };

        let user = self
            .repository
            .find_by_email(&self.pool, &new_user.email)
            .await?;

        match user {
            Some(_) => Err(AppError::Conflict("user already exists".to_string())),
            None => {
                let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

                let user = self.repository.register(&mut *tx, new_user).await?;
                self.roles_service
                    .assign_role(&mut tx, &user.id, &RoleEnum::User)
                    .await?;

                tx.commit().await.map_err(AppError::Database)?;
                Ok(user)
            }
        }
    }

    pub async fn login(&self, cmd: LoginCommand) -> Result<PublicAuthToken, AppError> {
        let user = self
            .repository
            .find_by_email(&self.pool, &cmd.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("wrong credentials".to_string()))?;

        let valid_password = verify_password(&cmd.password, &user.password)?;

        if !valid_password {
            return Err(AppError::Unauthorized("wrong credentials".to_string()));
        }

        let auth_token = self.get_auth_token(user.id).await?;

        Ok(auth_token)
    }

    async fn get_auth_token(&self, user_id: i64) -> Result<PublicAuthToken, AppError> {
        let check_token = self
            .repository
            .get_token_by_user_id(&self.pool, &user_id)
            .await?;

        let auth_token = match check_token.map(AuthToken::from) {
            Some(auth_token) if !auth_token.is_expired() => auth_token,
            _ => {
                let user_role = self.roles_service.get_user_role(&user_id).await?;

                let auth_token = AuthToken::new(&user_id, user_role.get_scopes());

                AuthToken::from(self.repository.save_token(&self.pool, &auth_token).await?)
            }
        };

        Ok(PublicAuthToken::from(auth_token))
    }

    pub async fn get_token_if_exist(
        &self,
        token: String,
    ) -> Result<Option<AuthTokenModel>, AppError> {
        self.repository.get_token(&self.pool, token).await
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|_| AppError::Internal("failed to hash password".to_string()))?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|_| AppError::Internal("invalid password hash".into()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
