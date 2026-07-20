use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{DocAddress, Index, IndexWriter, Score, doc, schema::*};

pub struct Search {
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
            line_field,
            index,
            writer,
            line_counter,
        })
    }

    pub fn index_lines(&mut self, file_path: PathBuf) -> Result<(), &'static str> {
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

#[cfg(test)]
mod tests {
    use crate::search::Search;
    use std::fs::write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn calculate_top_lines_should_round_up() {
        assert_eq!(Search::calculate_top_lines(10, 100), 10);
        assert_eq!(Search::calculate_top_lines(1, 100), 1);
        assert_eq!(Search::calculate_top_lines(50, 3), 2);
        assert_eq!(Search::calculate_top_lines(25, 7), 2);
        assert_eq!(Search::calculate_top_lines(100, 5), 5);
    }

    #[test]
    fn calculate_top_lines_should_never_return_zero() {
        assert_eq!(Search::calculate_top_lines(0, 100), 1);
        assert_eq!(Search::calculate_top_lines(0, 0), 1);
        assert_eq!(Search::calculate_top_lines(50, 0), 1);
    }

    #[test]
    fn new_should_create_search() {
        let search = Search::new();
        assert!(search.is_ok());
    }

    #[test]
    fn add_lines_should_index_file() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Lorem\nipsum\ndolor\n").unwrap();
        let mut search = Search::new().unwrap();
        assert!(search.index_lines(file.path().to_path_buf()).is_ok());
        assert_eq!(search.line_counter, 3);
    }

    #[test]
    fn add_lines_should_fail_for_missing_file() {
        let mut search = Search::new().unwrap();
        let result = search.index_lines(PathBuf::from("does_not_exist.txt"));
        assert_eq!(result, Err("cannot open the file"));
    }

    #[test]
    fn query_should_find_matching_documents() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Lorem ipsum\ndolor sit amet...\nLorem else\n").unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query("Lorem".to_string(), 100).unwrap();
        assert_eq!(result.len(), 2);
        let lines: Vec<String> = result.into_iter().map(|(_, l)| l).collect();
        assert!(lines.contains(&"Lorem ipsum".to_string()));
        assert!(lines.contains(&"Lorem else".to_string()));
    }

    #[test]
    fn query_should_return_empty_when_nothing_matches() {
        let file = NamedTempFile::new().unwrap();
        write(
            file.path(),
            "Lorem ipsum\ndolor sit amet,\nad ad enim eiusmod sed\n",
        )
        .unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query("none".to_string(), 100).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn query_should_return_error_for_invalid_query() {
        let file = NamedTempFile::new().unwrap();
        write(
            file.path(),
            "Lorem ipsum\ndolor sit amet,\nad ad enim eiusmod sed\n",
        )
        .unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query("\"".to_string(), 100);
        assert_eq!(result, Err("incorrect query syntax"));
    }

    #[test]
    fn query_should_limit_number_of_results() {
        let file = NamedTempFile::new().unwrap();
        write(
            file.path(),
            "apple one\napple two\napple three\napple four\n",
        )
        .unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query("apple".to_string(), 25).unwrap();
        assert_eq!(result.len(), 1);
    }
}
