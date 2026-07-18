use tantivy::{Index, IndexWriter, schema::*};

pub struct Search {
    schema: Schema,
    index: Index,
    writer: IndexWriter,
}

impl Search {
    pub fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("line", TEXT | STORED);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(50_000_000)?;
        Ok(Self{schema, index, writer})
    }
}
