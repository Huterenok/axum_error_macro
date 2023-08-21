mod transferred_data {
    use axum_error_macro::ErrorResponse;

    #[tokio::test]
    async fn right_transferred_data() {
        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }

        let msg1 = "Internal server error!!!";
        let msg2 = "Bad request!!!";

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            msg1.as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            msg2.as_bytes()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn wrong_transferred_data() {
        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 500, msg = "Internal server error!!!")]
            InternalServerError,
            #[error(code = 400, msg = "Bad request!!!")]
            BadRequest,
        }
        let msg = "Wrong error!!!";

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            msg.as_bytes()
        );

        assert_eq!(
            Error::BadRequest
                .into_response()
                .data()
                .await
                .unwrap()
                .unwrap(),
            msg.as_bytes()
        );
    }
}
