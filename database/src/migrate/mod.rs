use sea_orm_migration::prelude::*;

mod m_01_00_000;

pub(crate) struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m_01_00_000::Migration)]
    }
}

struct IdenVal(&'static str);

impl Iden for IdenVal {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

/// u32 auto increment not null "id" column
fn id() -> ColumnDef {
    let mut id = column("id");
    id.unsigned().auto_increment().not_null();
    id
}

fn column(name: &'static str) -> ColumnDef {
    ColumnDef::new(IdenVal(name))
}

fn create_table(name: &'static str) -> TableCreateStatement {
    let name = IdenVal(name);
    let mut table = Table::create();
    table.table(name);
    table
}
