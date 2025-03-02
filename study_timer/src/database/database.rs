use chrono::{Datelike, Utc};
use rusqlite::Connection;
use std::io::{self, ErrorKind};
pub struct DatabaseManager {
    conn: Connection,
}

impl DatabaseManager {
    pub fn new(db_path: &String) -> io::Result<Self> {
        let conn = Connection::open(db_path).map_err(DatabaseManager::to_io_error)?;
        Ok(Self { conn })
    }

    pub fn create_table(&self) -> io::Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS timer_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date DATE NOT NULL,
                time TIME NOT NULL,
                duration INTEGER NOT NULL,
                completed BOOLEAN NOT NULL
            )",
                [],
            )
            .map_err(DatabaseManager::to_io_error)?;
        Ok(())
    }

    pub fn insert_data(&self, duration: u64, completed: bool) -> io::Result<()> {
        // Get current date and time using chrono
        let current_date = Utc::now().format("%Y-%m-%d").to_string(); // YYYY-MM-DD
        let current_time = Utc::now().format("%H:%M:%S").to_string(); // HH:MM:SS

        // Prepare the SQL query
        let sql =
            "INSERT INTO timer_events (date, time, duration, completed) VALUES (?1, ?2, ?3, ?4)";

        // Execute the query with the correct types
        self.conn
            .execute(
                sql,
                &[
                    &current_date,
                    &current_time,
                    &duration as &dyn rusqlite::ToSql,
                    &completed as &dyn rusqlite::ToSql,
                ],
            )
            .map_err(DatabaseManager::to_io_error)?;
        Ok(())
    }

    pub fn get_all_data(&self) -> io::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, date, time, duration, completed FROM timer_events")
            .map_err(DatabaseManager::to_io_error)?;

        let data_points = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,    // id
                    row.get::<_, String>(1)?, // date
                    row.get::<_, String>(2)?, // time
                    row.get::<_, i32>(3)?,    // duration
                    row.get::<_, bool>(4)?,   // completed
                ))
            })
            .map_err(DatabaseManager::to_io_error)?;

        for data_point in data_points {
            match data_point {
                Ok((id, date, time, duration, completed)) => {
                    println!(
                        "ID: {}, Date: {}, Time: {}, Duration: {} seconds, Completed: {}",
                        id, date, time, duration, completed
                    );
                }
                Err(err) => {
                    eprintln!("Error retrieving data point: {}", err);
                }
            }
        }

        Ok(())
    }

    fn to_io_error(err: rusqlite::Error) -> io::Error {
        io::Error::new(ErrorKind::Other, format!("Database error: {}", err))
    }

    pub fn get_data(&self, weeks_ago: u32) -> io::Result<u64> {
        // Get current date in UTC
        let now = Utc::now();

        // Find the most recent Monday
        let days_since_monday = now.weekday().num_days_from_monday();
        let this_monday = now - chrono::Duration::days(days_since_monday as i64);

        // Calculate the start and end dates for the requested week
        let start_date = this_monday - chrono::Duration::weeks(weeks_ago as i64);
        let end_date = start_date + chrono::Duration::weeks(1);

        // Format dates for SQLite query
        let start_date_str = start_date.format("%Y-%m-%d").to_string();
        let end_date_str = end_date.format("%Y-%m-%d").to_string();

        // Prepare and execute query
        let sql = "
            SELECT COALESCE(SUM(duration), 0) as total_duration 
            FROM timer_events 
            WHERE date >= ?1 
            AND date < ?2
        ";

        let total_duration: i64 = self
            .conn
            .query_row(sql, &[&start_date_str, &end_date_str], |row| row.get(0))
            .map_err(DatabaseManager::to_io_error)?;

        Ok(total_duration as u64)
    }
}
