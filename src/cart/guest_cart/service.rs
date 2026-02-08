use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository::CartItemsRepository;
use crate::cart::guest_cart::dto::PublicGuestCart;
use crate::cart::guest_cart::repository::GuestCartRepository;
use crate::errors::error::AppError;
use crate::users::service::UserService;
use sqlx::PgPool;

pub struct GuestCartService {
    repository: GuestCartRepository,
    cart_items_repository: CartItemsRepository,
    user_service: UserService,
}

impl GuestCartService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: GuestCartRepository::new(pool.clone()),
            cart_items_repository: CartItemsRepository::new(pool.clone()),
            user_service: UserService::new(pool.clone()),
        }
    }

    pub async fn get_cart_by_hash(&self, hash: &str) -> Result<PublicGuestCart, AppError> {
        let user_hash = self
            .user_service
            .get_user_hash(hash)
            .await?
            .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

        let cart = self.repository.get_cart_by_user_hash(&user_hash.id).await?;

        let cart = match cart {
            Some(cart) => cart,
            None => {
                let cart = self.repository.create_hash_cart(&user_hash.id).await?;
                return Ok(PublicGuestCart::new_from_model(cart));
            }
        };

        let cart_items: Vec<CartItemModel> = self.cart_items_repository.get_items(&cart.id).await?;

        Ok(PublicGuestCart::new_with_items(cart, cart_items))
    }

    pub async fn get_cart_id_by_hash(&self, hash: &str) -> Result<i64, AppError> {
        let user_hash = self
            .user_service
            .get_user_hash(hash)
            .await?
            .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

        let cart_id = self.repository.get_cart_id(&user_hash.id).await?;

        match cart_id {
            Some(cart_id) => Ok(cart_id.id),
            None => {
                let cart = self.repository.create_hash_cart(&user_hash.id).await?;
                Ok(cart.id)
            }
        }
    }
}
