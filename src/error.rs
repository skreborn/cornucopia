use miette::Diagnostic;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum Error {
    Connection(#[from] crate::conn::error::Error),
    ReadQueries(#[from] crate::read_queries::error::Error),
    ParseQueries(#[from] crate::parser::error::Error),
    ValidateQueries(#[from] crate::validation::error::Error),
    Container(#[from] crate::container::error::Error),
    PrepareQueries(#[from] crate::prepare_queries::error::Error),
    NewMigration(#[from] NewMigrationError),
    RunMigration(#[from] crate::run_migrations::error::Error),
    ReadMigration(#[from] crate::read_migrations::error::Error),
    WriteCodeGenFile(#[from] WriteOutputError),
}

impl Error {
    pub fn report(self) -> String {
        format!("{:?}", miette::Report::from(self))
    }
}

#[derive(Debug, ThisError, Diagnostic)]
#[diagnostic(code(cornucopia::write_output))]
#[error("Could not write your queries to destination file `{file_path}`: ({err})")]
pub struct WriteOutputError {
    pub(crate) file_path: String,
    pub(crate) err: std::io::Error,
}

#[derive(Debug, ThisError, Diagnostic)]
#[diagnostic(code(cornucopia::new_migration))]
#[error("Could not create new migration `{file_path}`: ({err})")]
pub struct NewMigrationError {
    pub(crate) file_path: String,
    pub(crate) err: std::io::Error,
}
