use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Shipment {
    pub id: u64,
    pub code: String,
    pub shipper: String,
    pub airport: String,
    pub incoterms: String,
    pub notes: Option<String>,
}

pub fn shipment_table_setup(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shipment (
            id         INTEGER PRIMARY KEY,
            code       TEXT NOT NULL,
            shipper    TEXT NOT NULL,
            airport    TEXT NOT NULL,
            incoterms  TEXT NOT NULL,
            notes      TEXT
        )",
        (),
    )?;
    Ok(())
}

pub fn create(conn: &Connection, shipment: &Shipment) -> Result<()> {
    conn.execute(
        "INSERT INTO shipment (
            id,
            code,
            shipper,
            airport,
            incoterms,
            notes
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &shipment.id,
            &shipment.code,
            &shipment.shipper,
            &shipment.airport,
            &shipment.incoterms,
            &shipment.notes,
        ),
    )?;
    Ok(())
}
