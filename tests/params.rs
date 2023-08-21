mod params {
    use axum_error_macro::IntoResponse;

    #[tokio::test]
    async fn right_params() {
        #[derive(IntoResponse)]
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
    async fn struct_params() {
        #[derive(Debug)]
        struct User {
            username: String,
        }

        #[derive(IntoResponse)]
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
