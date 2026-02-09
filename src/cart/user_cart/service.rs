use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository::CartItemsRepository;
use crate::cart::user_cart::dto::PublicUserCart;
use crate::cart::user_cart::repository::UserCartRepository;
use crate::errors::error::AppError;
use crate::traits::IsRepository;
use crate::users::service::UserService;
use sqlx::PgPool;

pub struct UserCartService {
    repository: UserCartRepository,
    cart_items_repository: CartItemsRepository,
    user_service: UserService,
}

impl UserCartService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: UserCartRepository::new(pool.clone()),
            cart_items_repository: CartItemsRepository::new(pool.clone()),
            user_service: UserService::new(pool.clone()),
        }
    }

    pub async fn get_cart_by_user(&self, user_id: &i64) -> Result<PublicUserCart, AppError> {
        let user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

        let cart = self.repository.get_cart_by_user_id(&user.id).await?;

        let cart = match cart {
            Some(cart) => cart,
            None => {
                let cart = self.repository.create_user_cart(&user.id).await?;
                return Ok(PublicUserCart::new_from_model(cart));
            }
        };

        let cart_items: Vec<CartItemModel> = self.cart_items_repository.get_items(&cart.id).await?;

        Ok(PublicUserCart::new_with_items(cart, cart_items))
    }

    pub async fn get_cart_id_by_user(&self, user_id: &i64) -> Result<i64, AppError> {
        let user = self
            .user_service
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

        let cart_id = self.repository.get_cart_id(&user.id).await?;

        match cart_id {
            Some(cart_id) => Ok(cart_id.id),
            None => {
                let cart = self.repository.create_user_cart(&user.id).await?;
                Ok(cart.id)
            }
        }
    }
}
