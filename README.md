Installation:

```toml
[dependencies]
axum_error_macro = { version = "0.1.5" }
```

Simple example

```rust
use axum_error_macro::IntoResponse;
use axum::response::Response;

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

fn server_error_handler() -> Response {
  return Error::InternalServerError.into_response();
}

fn user_by_id_handler() -> Response {
  return Error::UserByIdNotFound(1).into_response();
}

fn user_by_username_and_role_handler() -> Response {
  return Error::UserByUsernameAndRoleNotFound("Bebra".into(), "ADMIN".into()).into_response();
}

fn user_handler() -> Response {
  let user = User {
    username: "Bebra".into()
  };
  return Error::UserNotFound(user).into_response();
}
```

Returned data will be in this format:

```json
{
  "message": "Internal Server Error!!!"
}
```
