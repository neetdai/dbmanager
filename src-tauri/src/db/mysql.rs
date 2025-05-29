use mysql_async::{Pool, PoolOpts};

#[derive(Debug)]
pub(crate) struct MysqlAdapter {
    pool: Pool,
}

impl MysqlAdapter {
    // pub(crate) fn
}