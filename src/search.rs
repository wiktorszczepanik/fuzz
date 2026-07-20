use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{DocAddress, Index, IndexWriter, Score, doc, schema::*};

pub struct Search {
    schema: Schema,
    line_field: Field,
    index: Index,
    writer: IndexWriter,
    line_counter: usize,
}

impl Search {
    pub fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        let line_field = schema_builder.add_text_field("line", TEXT | STORED);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(50_000_000)?;
        let line_counter = 0usize;
        Ok(Self {
            schema,
            line_field,
            index,
            writer,
            line_counter,
        })
    }

    pub fn add_lines(&mut self, file_path: PathBuf) -> Result<(), &'static str> {
        let file = File::open(file_path).map_err(|_| "cannot open the file")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.line_counter += 1;
            let line = line.map_err(|_| "cannot read line")?;
            self.writer
                .add_document(doc!(self.line_field => line))
                .map_err(|_| "cannot add document")?;
        }
        self.writer.commit().map_err(|_| "cannot commit index")?;
        Ok(())
    }

    pub fn query(&mut self, text: String, top: u8) -> Result<Vec<(Score, String)>, &'static str> {
        let reader = self
            .index
            .reader()
            .map_err(|_| "could not setup query reader")?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![self.line_field]);
        let query = query_parser
            .parse_query(text.as_str())
            .map_err(|_| "incorrect query syntax")?;
        let top_lines = Self::calculate_top_lines(top, self.line_counter);
        let top_docs: Vec<(Score, DocAddress)> = searcher
            .search(&query, &TopDocs::with_limit(top_lines).order_by_score())
            .map_err(|_| "docs selection error")?;
        let mut results = Vec::with_capacity(top_docs.len());
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc::<TantivyDocument>(doc_address)
                .map_err(|_| "retrieved document error")?;
            if let Some(line) = retrieved_doc
                .get_first(self.line_field)
                .and_then(|v| v.as_str())
            {
                results.push((score, line.to_string()));
            }
        }
        Ok(results)
    }

    fn calculate_top_lines(top_percent: u8, lines: usize) -> usize {
        let top: usize = top_percent as usize;
        std::cmp::max(1, (top * lines + 99) / 100)
    }
}
