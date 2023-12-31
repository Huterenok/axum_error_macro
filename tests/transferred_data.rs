mod transferred_data {
    use axum_error_macro::ErrorResponse;
    use hyper::body::HttpBody;
    use serde_json::json;

    #[tokio::test]
    async fn right_transferred_data_json() {
        #[derive(ErrorResponse)]
        #[error_format("application/json")]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }

        let msg1 = json!({
            "message": "Internal server error!!!"
        });
        let msg2 = json!({
            "message": "Bad request!!!"
        });

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg1.to_string().as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg2.to_string().as_bytes()
        );
    }

    #[tokio::test]
    async fn right_transferred_data_text() {
        #[derive(ErrorResponse)]
        #[error_format("text/plain")]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }

        let msg1 = "Internal server error!!!".to_string();
        let msg2 = "Bad request!!!".to_string();

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg1.as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg2.as_bytes()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn wrong_transferred_data_json() {
        #[derive(ErrorResponse)]
        #[error_format("application/json")]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }
        let msg = json!({
            "message": "Wrong error!!!"
        });

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg.to_string().as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg.to_string().as_bytes()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn wrong_transferred_data_text() {
        #[derive(ErrorResponse)]
        #[error_format("text/plain")]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }
        let msg = "Wrong error!!!".to_string();

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg.as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
            msg.to_string().as_bytes()
        );
    }
}
