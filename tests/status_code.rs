#[cfg(test)]
mod status_code {
    use axum_error_macro::IntoResponse;
    use hyper::StatusCode;

    #[test]
    fn right_status_code() {
        #[derive(IntoResponse)]
        enum Error {
            #[error(code = 500, msg = "123")]
            InternalServerError,
            #[error(code = 400, msg = "123")]
            BadRequest,
        }

        assert_eq!(
            Error::InternalServerError.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            Error::BadRequest.into_response().status(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    #[should_panic]
    fn wrong_status_code() {
        #[derive(IntoResponse)]
        enum Error {
            #[error(code = 500, msg = "123")]
            InternalServerError,
            #[error(code = 400, msg = "123")]
            BadRequest,
        }

        assert_eq!(
            Error::BadRequest.into_response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
