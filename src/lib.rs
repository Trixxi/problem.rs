pub mod providers;
pub mod macros;

#[derive(Debug, Clone, PartialEq)]
pub enum ProblemErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Timeout,
    Conflict,
    PreconditionFailed,
    PayloadTooLarge,
    MethodNotSupported,
    ClientClosedRequest,
    InternalServerError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    pub title: String,
    pub detail: String,
    pub status: ProblemErrorCode,
}
impl Problem {
    pub fn not_found( detail: impl Into<String> ) -> Problem {
        Problem { title: "Not found".to_string(), detail: detail.into(), status: ProblemErrorCode::NotFound }
    }
    pub fn bad_request( detail: impl Into<String> ) -> Problem {
        Problem { title: "Bad request".to_string(), detail: detail.into(), status: ProblemErrorCode::NotFound }
    }
    pub fn internal_server_error( detail: impl Into<String> ) -> Problem {
        Problem { title: "Internal server error".to_string(), detail: detail.into(), status: ProblemErrorCode::InternalServerError }
    }
    pub fn forbidden( detail: impl Into<String> ) -> Problem {
        Problem { title: "Forbidden".to_string(), detail: detail.into(), status: ProblemErrorCode::Forbidden }
    }
    pub fn unauthorized( detail: impl Into<String> ) -> Problem {
        Problem { title: "Unauthorized".to_string(), detail: detail.into(), status: ProblemErrorCode::Unauthorized }
    }
}


// a macro to generate a Problem and return as Err\
#[macro_export]
macro_rules! problem {
    ($status:ident, $detail:expr) => {
        Err(crate::Problem:: $status ( $detail.to_string() ))
    };
}

// tests for the macro
#[cfg(test)]
mod tests {
    use super::*;
    fn return_problem() -> ProblemResult<String>{
        problem!(not_found, "Not found")
    }

    fn problem_macros() -> ProblemResult<String> {
        not_found!("Not found {}", 123)
    }

    #[test]
    fn test_problem_macro() {
        let result = return_problem();
        assert!(result.is_err());
        let problem = result.unwrap_err();
        assert_eq!(problem.status, ProblemErrorCode::NotFound);
        assert_eq!(problem.detail, "Not found");
    }
}


impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.detail)
    }
}

pub type ProblemResult<T> = Result<T, Problem>;

impl std::error::Error for Problem {
    fn description(&self) -> &str {
        &self.title
    }
}

// convert std::Error to Problem
impl From<std::io::Error> for Problem {
    fn from(e: std::io::Error) -> Self {
        Problem {
            title: "IO Error".to_string(),
            detail: e.to_string(),
            status: ProblemErrorCode::InternalServerError
        }
    }
}

// impl option for Problem
impl From<Option<Problem>> for Problem {
    fn from(e: Option<Problem>) -> Self {
        match e {
            Some(p) => p,
            None => Problem {
                title: "No Error".to_string(),
                detail: "No Error".to_string(),
                status: ProblemErrorCode::InternalServerError
            }
        }
    }
}

#[macro_export]
macro_rules! unauthorized {
( $( $p:tt ),* ) => {{
    Err(Problem::unauthorized(format!($($p),*)))
}};
}

#[macro_export]
macro_rules! forbidden {
( $( $p:tt ),* ) => {{
    Err(Problem::forbidden(format!($($p),*)))
}};
}

#[macro_export]
macro_rules! not_found {
( $( $p:tt ),* ) => {{
    Err(Problem::not_found(format!($($p),*)))
}};
}

#[macro_export]
macro_rules! bad_request {
( $( $p:tt ),* ) => {{
    Err(Problem::bad_request(format!($($p),*)))
}};
}

#[macro_export]
macro_rules! internal_server_error {
( $( $p:tt ),* ) => {{
    Err(Problem::internal_server_error(format!($($p),*)))
}};
}