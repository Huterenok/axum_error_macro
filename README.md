Installation:

```toml
[dependencies]
axum_error_macro = { version = "0.1.0" }
```

Simple example

```rust
// A trait that the Validate derive will impl
use axum_error_macro::IntoResponse;

#[derive(Debug, Validate, Deserialize)]
#[derive(IntoResponse)]
enum Error {
  #[error(code = 500, msg = "Internal Server Error!!!")]
  InternalServerError,
  
  #[error(code = 400, msg = "Bad Request!!!")]
  BadRequest,
  
  #[error(code = 404, msg = "User by {} id was not found")]
  UserByIdNotFound(u32),
  
  #[error(code = 404, msg = "User by {} username with {} role was not found")]
  UserByUsernameAndRoleNotFound(String, String),
  
  #[error(code = 404, msg = "User {:?} was not found")]
  UserNotFound(User)
}

#[derive(Debug)]
struct User {
  username: String
}

fn server_error_handler() -> Error {
  return Error::InternalServerError.into_response();
}

fn user_by_id_handler() -> Error {
  return Error::UserByIdNotFound(1).into_response();
}

fn user_by_username_and_role_handler() -> Error {
  return Error::UserByUsernameAndRoleNotFound("Bebra".into(), "ADMIN".into()).into_response();
}

fn user_handler() -> Error {
  let user = User {
    username: "Bebra".into()
  };
  return Error::UserNotFound(user).into_response();
}
```
