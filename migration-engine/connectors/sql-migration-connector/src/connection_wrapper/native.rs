use super::{SqlError, SqlResult};
use migration_connector::{ConnectorError, ConnectorResult};
use quaint::{
    connector::{Mysql, MysqlUrl, PostgreSql, PostgresUrl},
    error::Error as QuaintError,
    prelude::{ConnectionInfo, Query, Queryable, ResultSet},
    single::Quaint,
};
use std::sync::Arc;
use user_facing_errors::KnownError;

pub(crate) async fn connect(connection_string: &str) -> ConnectorResult<Connection> {
    let connection_info = ConnectionInfo::from_url(connection_string).map_err(|err| {
        let details = user_facing_errors::quaint::invalid_connection_string_description(&err.to_string());
        KnownError::new(user_facing_errors::common::InvalidConnectionString { details })
    })?;

    if let ConnectionInfo::Postgres(url) = &connection_info {
        return quaint::connector::PostgreSql::new(url.clone())
            .await
            .map(|conn| Connection::new_postgres(conn, url.clone()))
            .map_err(|err| quaint_error_to_connector_error(err, &connection_info));
    }

    if let ConnectionInfo::Mysql(url) = &connection_info {
        return quaint::connector::Mysql::new(url.clone())
            .await
            .map(|conn| Connection::new_mysql(conn, url.clone()))
            .map_err(|err| quaint_error_to_connector_error(err, &connection_info));
    }

    let connection = Quaint::new(connection_string)
        .await
        .map_err(|err| quaint_error_to_connector_error(err, &connection_info))?;

    Ok(Connection::new_generic(connection))
}

pub(crate) fn quaint_error_to_connector_error(error: QuaintError, connection_info: &ConnectionInfo) -> ConnectorError {
    match user_facing_errors::quaint::render_quaint_error(error.kind(), connection_info) {
        Some(user_facing_error) => user_facing_error.into(),
        None => {
            let msg = error
                .original_message()
                .map(String::from)
                .unwrap_or_else(|| error.to_string());
            ConnectorError::from_msg(msg)
        }
    }
}

fn sql_error(quaint_error: QuaintError, connection_info: &ConnectionInfo) -> SqlError {
    let error_code = quaint_error.original_code().map(String::from);
    super::SqlError {
        connector_error: quaint_error_to_connector_error(quaint_error, connection_info),
        _src_position: None,
        _src_statement: None,
        error_code,
    }
}

/// An internal helper for the SQL connector. It wraps a `Quaint` struct and
/// exposes a similar API, with additional error handling to return
/// `ConnectorResult`s.
#[derive(Clone)]
pub(crate) struct Connection(ConnectionInner, ConnectionInfo);

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<CONNECTION>") // we do not want to leak secrets
    }
}

#[derive(Clone)]
enum ConnectionInner {
    Postgres(Arc<(quaint::connector::PostgreSql, PostgresUrl)>),
    Mysql(Arc<(quaint::connector::Mysql, MysqlUrl)>),
    Generic(Quaint),
}

impl Connection {
    pub(crate) fn new_generic(quaint: Quaint) -> Self {
        let connection_info = quaint.connection_info().to_owned();
        Connection(ConnectionInner::Generic(quaint), connection_info)
    }

    fn new_postgres(conn: PostgreSql, url: PostgresUrl) -> Self {
        Connection(
            ConnectionInner::Postgres(Arc::new((conn, url.clone()))),
            ConnectionInfo::Postgres(url),
        )
    }

    fn new_mysql(conn: Mysql, url: MysqlUrl) -> Self {
        Connection(
            ConnectionInner::Mysql(Arc::new((conn, url.clone()))),
            ConnectionInfo::Mysql(url),
        )
    }

    pub(crate) fn connection_info(&self) -> &ConnectionInfo {
        &self.1
    }

    pub(crate) fn queryable(&self) -> &dyn Queryable {
        match &self.0 {
            ConnectionInner::Postgres(pg) => &pg.0,
            ConnectionInner::Mysql(my) => &my.0,
            ConnectionInner::Generic(q) => q,
        }
    }

    pub(crate) async fn query(&self, query: impl Into<Query<'_>>) -> SqlResult<ResultSet> {
        self.queryable()
            .query(query.into())
            .await
            .map_err(|quaint_error| sql_error(quaint_error, self.connection_info()))
    }

    pub(crate) async fn query_raw(&self, sql: &str, params: &[quaint::Value<'_>]) -> SqlResult<ResultSet> {
        self.queryable()
            .query_raw(sql, params)
            .await
            .map_err(|quaint_error| sql_error(quaint_error, self.connection_info()))
    }

    pub(crate) async fn raw_cmd(&self, sql: &str) -> SqlResult<()> {
        self.queryable()
            .raw_cmd(sql)
            .await
            .map_err(|quaint_error| sql_error(quaint_error, self.connection_info()))
    }

    pub(crate) async fn version(&self) -> SqlResult<Option<String>> {
        self.queryable()
            .version()
            .await
            .map_err(|quaint_error| sql_error(quaint_error, self.connection_info()))
    }

    pub(crate) fn unwrap_postgres(&self) -> &(PostgreSql, PostgresUrl) {
        match &self.0 {
            ConnectionInner::Postgres(inner) => inner,
            _ => panic!("unexpected data in Connection::unwrap_postgres()"),
        }
    }

    pub(crate) fn unwrap_mysql(&self) -> &(Mysql, MysqlUrl) {
        match &self.0 {
            ConnectionInner::Mysql(inner) => &**inner,
            _ => panic!("unexpected data in Connection::unwrap_mysql()"),
        }
    }
}
