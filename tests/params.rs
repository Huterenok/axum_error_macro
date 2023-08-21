mod params {
    use axum_error_macro::ErrorResponse;

    #[tokio::test]
    async fn right_param() {
        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 404, msg = "Post by {} id was not found")]
            PostByIdNotFound(u32),
            #[error(code = 404, msg = "User by {} username was not found")]
            UserByUsernameNotFound(String),
        }
        let id = 12;
        let username = "Bebra";

        let post_error_msg = format!("Post by {} id was not found", id);
        let user_error_msg = format!("User by {} username was not found", username);

        assert_eq!(
            Error::PostByIdNotFound(id)
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            post_error_msg.as_bytes()
        );

        assert_eq!(
            Error::UserByUsernameNotFound(username.into())
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            user_error_msg.as_bytes()
        );
    }

    #[tokio::test]
    async fn right_multiply_params() {
        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 404, msg = "User by {} username with {} role was not found")]
            UserByUsernameAndRoleNotFound(String, String),
        }
        let role = "ADMIN";
        let username = "Bebra";

        let error_msg = format!(
            "User by {} username with {} role was not found",
            username, role
        );

        assert_eq!(
            Error::UserByUsernameAndRoleNotFound(username.into(), role.into())
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            error_msg.as_bytes()
        );
    }

    #[tokio::test]
    async fn struct_param() {
        #[derive(Debug)]
        struct User {
            username: String,
        }

        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 404, msg = "User {:?}  was not found")]
            UserNotFound(User),
        }
        let user = User {
            username: "bebra".into(),
        };

        let error_msg = format!("User {:?}  was not found", user);

        assert_eq!(
            Error::UserNotFound(user)
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            error_msg.as_bytes()
        );
    }
}
