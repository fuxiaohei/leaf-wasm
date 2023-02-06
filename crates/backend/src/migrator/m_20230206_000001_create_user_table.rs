use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230206_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LeafUser::Table)
                    .col(
                        ColumnDef::new(LeafUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LeafUser::Name).char_len(32).not_null())
                    .col(ColumnDef::new(LeafUser::Email).char_len(128).not_null())
                    .col(ColumnDef::new(LeafUser::Password).char_len(256).not_null())
                    .col(
                        ColumnDef::new(LeafUser::CreatedAt)
                            .integer_len(12)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(LeafUser::UpdatedAt)
                            .integer_len(12)
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LeafUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum LeafUser {
    Table,
    Id,
    Name,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
