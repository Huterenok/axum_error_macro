mod format {
    use axum_error_macro::ErrorResponse;

    #[test]
    fn json_format() {
        #[derive(ErrorResponse)]
        #[error_format("application/json")]
        enum Error {
            #[error(code = 500, msg = "123")]
            InternalServerError,
            #[error(code = 400, msg = "123")]
            BadRequest,
        }

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "application/json"
        );
        assert_eq!(
            Error::BadRequest
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "application/json"
        );
    }

    #[test]
    fn text_format() {
        #[derive(ErrorResponse)]
        #[error_format("text/plain")]
        enum Error {
            #[error(code = 500, msg = "123")]
            InternalServerError,
            #[error(code = 400, msg = "123")]
            BadRequest,
        }

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "text/plain"
        );
        assert_eq!(
            Error::BadRequest
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "text/plain"
        );
    }

    #[test]
    fn implicit_text_format() {
        #[derive(ErrorResponse)]
        enum Error {
            #[error(code = 500, msg = "123")]
            InternalServerError,
            #[error(code = 400, msg = "123")]
            BadRequest,
        }

        assert_eq!(
            Error::InternalServerError
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "text/plain"
        );
        assert_eq!(
            Error::BadRequest
                .into_response()
                .headers()
                .get("Content-Type")
                .unwrap(),
            "text/plain"
        );
    }
}
