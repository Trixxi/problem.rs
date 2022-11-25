use crate::Problem;
use crate::ProblemErrorCode;


// only when mongodb is enabled
#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for Problem {
    fn from(e: mongodb::error::Error) -> Self {
        Problem {
            title: "MongoDB Error".to_string(),
            //detail: "Database is being dumb".to_string(),
            detail: e.to_string(), /** Should not send this, @FIXME */
            status: ProblemErrorCode::InternalServerError
        }
    }
}
// only when rspc is enabled
#[cfg(feature = "rspc")]
impl From<Problem> for rspc::Error {
    // 
    fn from(p: Problem) -> Self {
        /* todo: parse */
        rspc::Error::new(ProblemErrorCode::BadRequest, p.detail.into())
    }
}
