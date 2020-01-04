use super::super::engine::Transaction;
use super::super::types::Row;
use super::{Context, Executor};
use crate::Error;

/// A delete executor
pub struct Delete;

impl Delete {
    pub fn execute<T: Transaction>(
        ctx: &mut Context<T>,
        mut source: Box<dyn Executor>,
        table: String,
    ) -> Result<Box<dyn Executor>, Error> {
        let table = ctx
            .txn
            .read_table(&table)?
            .ok_or_else(|| Error::Value(format!("Table {} not found", table)))?;
        while let Some(row) = source.fetch()? {
            ctx.txn.delete(&table.name, &table.row_key(&row)?)?
        }
        Ok(Box::new(Self))
    }
}

impl Executor for Delete {
    fn columns(&self) -> Vec<String> {
        Vec::new()
    }

    fn fetch(&mut self) -> Result<Option<Row>, Error> {
        Ok(None)
    }
}
