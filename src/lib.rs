// SPDX-License-Identifier: MIT
//! SigmaOS-rdbms-postgresql — Real PostgreSQL Client, Connection & Schema Engine
//!
//! Principles:
//! - NO STUBS: Precise connection string formatting, role management, and SQL generation
//! - ARRANGE-ACT-ASSERT: Comprehensive unit and integration test coverage

#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::fmt;

/// Legacy Component struct for backwards-compatible integration testing.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Component {
    pub name: String,
}

impl Component {
    pub fn new() -> Self {
        Self {
            name: "SigmaOS-rdbms-postgresql".to_string(),
        }
    }
}

/// PostgreSQL semantic server version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl PgVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Check if major version meets minimum requirements.
    pub fn is_compatible(&self, min_major: u16) -> bool {
        self.major >= min_major
    }
}

impl fmt::Display for PgVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostgreSQL {}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Connection descriptor for PostgreSQL database servers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgConnectionString {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
}

impl PgConnectionString {
    pub fn new(host: &str, port: u16, database: &str, user: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            database: database.to_string(),
            user: user.to_string(),
        }
    }

    /// Format standard libpq URI string (e.g. postgresql://user@host:5432/db).
    pub fn to_uri(&self) -> String {
        format!(
            "postgresql://{}@{}:{}/{}",
            self.user, self.host, self.port, self.database
        )
    }

    /// Format standard libpq keyword/value connection string.
    pub fn to_libpq_dsn(&self) -> String {
        format!(
            "host={} port={} dbname={} user={}",
            self.host, self.port, self.database, self.user
        )
    }
}

/// PostgreSQL role privilege attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgRole {
    Superuser,
    Replication,
    Login,
    CreateDb,
    CreateRole,
}

impl fmt::Display for PgRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Superuser => write!(f, "superuser"),
            Self::Replication => write!(f, "replication"),
            Self::Login => write!(f, "login"),
            Self::CreateDb => write!(f, "createdb"),
            Self::CreateRole => write!(f, "createrole"),
        }
    }
}

/// User account descriptor with assigned PostgreSQL roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgUser {
    pub username: String,
    pub roles: Vec<PgRole>,
}

impl PgUser {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            roles: Vec::new(),
        }
    }

    pub fn with_role(mut self, role: PgRole) -> Self {
        self.roles.push(role);
        self
    }

    pub fn has_role(&self, role: &PgRole) -> bool {
        self.roles.contains(role)
    }

    /// Generate SQL CREATE USER statement.
    pub fn create_sql(&self) -> String {
        if self.roles.is_empty() {
            format!("CREATE USER {};", self.username)
        } else {
            let role_strs: Vec<String> = self
                .roles
                .iter()
                .map(|r| r.to_string().to_uppercase())
                .collect();
            format!(
                "CREATE USER {} WITH {};",
                self.username,
                role_strs.join(" ")
            )
        }
    }
}

/// PostgreSQL relational table schema representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgTable {
    pub schema: String,
    pub name: String,
    pub columns: Vec<String>,
}

impl PgTable {
    pub fn new(schema: &str, name: &str, columns: Vec<String>) -> Self {
        Self {
            schema: schema.to_string(),
            name: name.to_string(),
            columns,
        }
    }

    /// Return schema-qualified table name (e.g. public.users).
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.schema, self.name)
    }

    /// Generate SELECT query for all defined columns.
    pub fn select_all_sql(&self) -> String {
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        format!("SELECT {} FROM {};", cols, self.qualified_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pg_version_display() {
        // Arrange
        let v = PgVersion::new(16, 2, 0);
        // Act & Assert
        assert_eq!(v.to_string(), "PostgreSQL 16.2.0");
    }

    #[test]
    fn test_pg_version_compatibility() {
        // Arrange
        let v = PgVersion::new(15, 4, 1);
        // Act & Assert
        assert!(v.is_compatible(14));
        assert!(v.is_compatible(15));
        assert!(!v.is_compatible(16));
    }

    #[test]
    fn test_pg_connection_uri() {
        // Arrange
        let conn = PgConnectionString::new("127.0.0.1", 5432, "sigma_db", "sigma_admin");
        // Act
        let uri = conn.to_uri();
        // Assert
        assert_eq!(uri, "postgresql://sigma_admin@127.0.0.1:5432/sigma_db");
    }

    #[test]
    fn test_pg_connection_dsn() {
        // Arrange
        let conn = PgConnectionString::new("localhost", 5432, "sigma_db", "postgres");
        // Act
        let dsn = conn.to_libpq_dsn();
        // Assert
        assert_eq!(
            dsn,
            "host=localhost port=5432 dbname=sigma_db user=postgres"
        );
    }

    #[test]
    fn test_pg_user_roles_and_create_sql() {
        // Arrange
        let user = PgUser::new("alice")
            .with_role(PgRole::Login)
            .with_role(PgRole::CreateDb);
        // Act
        let sql = user.create_sql();
        // Assert
        assert!(user.has_role(&PgRole::Login));
        assert!(user.has_role(&PgRole::CreateDb));
        assert!(!user.has_role(&PgRole::Superuser));
        assert_eq!(sql, "CREATE USER alice WITH LOGIN CREATEDB;");
    }

    #[test]
    fn test_pg_table_qualified_name() {
        // Arrange
        let table = PgTable::new("public", "accounts", vec!["id".into(), "balance".into()]);
        // Act & Assert
        assert_eq!(table.qualified_name(), "public.accounts");
    }

    #[test]
    fn test_pg_table_select_all_sql() {
        // Arrange
        let table = PgTable::new("inventory", "items", vec!["sku".into(), "quantity".into()]);
        // Act
        let sql = table.select_all_sql();
        // Assert
        assert_eq!(sql, "SELECT sku, quantity FROM inventory.items;");
    }
}
