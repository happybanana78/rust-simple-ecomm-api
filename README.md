# E-Commerce API (Learning Project)

A RESTful e-commerce backend API built with Rust, Actix-web, and SQLx.

## Tech Stack

- **Rust**
- **Actix-web** - Web framework
- **SQLx** - Async database driver with compile-time checked queries
- **PostgreSQL** - Database
- **Argon2** - Password hashing
- **Validator** - Request validation

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.92.0 or later)
- [PostgreSQL](https://www.postgresql.org/download/) (14+ recommended)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for migrations

### Installing SQLx CLI

`cargo install sqlx-cli --no-default-features --features postgres`

## Getting Started

### 1. Clone the repository

`git clone https://github.com/happybanana78/rust-simple-ecomm-api.git` \
\
`cd ecomm`

### 2. Set up environment variables

Create a `.env` file in the project root: \
\
`DATABASE_URL=postgres://username:password@localhost:5432/ecomm`

Replace `username` and `password` with your PostgreSQL credentials.

### 3. Create the database

`cargo sqlx database create`

### 4. Run migrations

`cargo sqlx migrate run`

### 5. Seed the database

Run the seeder to populate initial data (roles, etc.): \
\
`cargo run --bin seed`

### 6. Run the application

`cargo run`

The server will start at `http://127.0.0.1:8080`

## API Endpoints

### Authentication

| Method | Endpoint       | Description         |
|--------|----------------|---------------------|
| POST   | /auth/register | Register new user   |
| POST   | /auth/login    | Login and get token |

### Products (Public)

| Method | Endpoint           | Description       |
|--------|--------------------|-------------------|
| GET    | /products/list     | List all products |
| GET    | /products/get/{id} | Get product by ID |

### Admin Products (Protected)

| Method | Endpoint                    | Description       |
|--------|-----------------------------|-------------------|
| GET    | /admin/products/list        | List all products |
| GET    | /admin/products/get/{id}    | Get product by ID |
| POST   | /admin/products/create      | Create product    |
| PUT    | /admin/products/update/{id} | Update product    |
| DELETE | /admin/products/delete/{id} | Delete product    |

### Cart (Authenticated User)

| Method | Endpoint          | Description          |
|--------|-------------------|----------------------|
| GET    | /cart/user/get    | Get user cart        |
| POST   | /cart/user/add    | Add item to cart     |
| PUT    | /cart/user/update | Update item quantity |
| DELETE | /cart/user/remove | Remove item          |

### Cart (Guest)

| Method | Endpoint           | Description          |
|--------|--------------------|----------------------|
| GET    | /cart/guest/get    | Get guest cart       |
| POST   | /cart/guest/add    | Add item to cart     |
| PUT    | /cart/guest/update | Update item quantity |
| DELETE | /cart/guest/remove | Remove item          |

## Authentication

Protected endpoints require a Bearer token in the Authorization header

## Credits

#### This README file was generated with the help of AI
