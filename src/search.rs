use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query};
use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
use tantivy::{DocAddress, Index, IndexWriter, Score, doc, schema::*};

pub struct Search {
    line_field: Field,
    line_number_field: Field,
    index: Index,
    writer: IndexWriter,
    line_counter: usize,
}

impl Search {

    pub fn new() -> tantivy::Result<Self> {
        let mut schema_builder = Schema::builder();
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer("lowercase")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();
        let line_field = schema_builder.add_text_field("line", text_options);
        let line_number_field = schema_builder.add_u64_field("line_number", STORED);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        index.tokenizers().register(
            "lowercase",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(LowerCaser)
                .build(),
        );
        let writer = index.writer(50_000_000)?;
        Ok(Self {
            line_field,
            line_number_field,
            index,
            writer,
            line_counter: 0,
        })
    }

    pub fn index_lines(&mut self, file_path: PathBuf) -> Result<(), &'static str> {
        let file = File::open(file_path).map_err(|_| "cannot open the file")?;
        let reader = BufReader::new(file);
        for (line_number, line) in reader.lines().enumerate() {
            let line = line.map_err(|_| "cannot read line")?;
            self.line_counter += 1;
            self.writer
                .add_document(
                    doc!(self.line_field => line, self.line_number_field => (line_number + 1) as u64),
                )
                .map_err(|_| "cannot add document")?;
        }
        self.writer.commit().map_err(|_| "cannot commit index")?;
        Ok(())
    }

    pub fn query(
        &mut self,
        text: &String,
        top: u8,
    ) -> Result<Vec<(usize, f64, String)>, &'static str> {
        let reader = self
            .index
            .reader()
            .map_err(|_| "could not setup query reader")?;
        let searcher = reader.searcher();
        let query = self.subqueries(text.as_str());
        let top_lines = Self::calculate_top_lines(top, self.line_counter);
        let top_docs: Vec<(Score, DocAddress)> = searcher
            .search(&query, &TopDocs::with_limit(top_lines).order_by_score())
            .map_err(|_| "docs selection error")?;
        let mut results = Vec::with_capacity(top_docs.len());
        for (_, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc::<TantivyDocument>(doc_address)
                .map_err(|_| "retrieved document error")?;
            let line = retrieved_doc
                .get_first(self.line_field)
                .and_then(|v| v.as_str());
            let line_number = retrieved_doc
                .get_first(self.line_number_field)
                .and_then(|v| v.as_u64());
            if let (Some(line), Some(line_number)) = (line, line_number) {
                results.push((line_number as usize, 0.0, line.to_string()));
            }
        }
        Ok(results)
    }

    fn calculate_top_lines(top_percent: u8, lines: usize) -> usize {
        let top: usize = top_percent as usize;
        std::cmp::max(1, (top * lines + 99) / 100)
    }

    fn subqueries(&self, text: &str) -> BooleanQuery {
        let subqueries = text
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                (
                    Occur::Should,
                    Box::new(FuzzyTermQuery::new(
                        Term::from_field_text(self.line_field, word),
                        2,
                        true,
                    )) as Box<dyn Query>,
                )
            })
            .collect();
        BooleanQuery::new(subqueries)
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
    fn index_lines_should_index_file() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Lorem\nipsum\ndolor\n").unwrap();
        let mut search = Search::new().unwrap();
        assert!(search.index_lines(file.path().to_path_buf()).is_ok());
        assert_eq!(search.line_counter, 3);
    }

    #[test]
    fn index_lines_should_fail_for_missing_file() {
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
        let result = search.query(&"Lorem".to_string(), 100).unwrap();
        assert_eq!(result.len(), 2);
        let lines: Vec<String> = result.into_iter().map(|(_, _, line)| line).collect();
        assert!(lines.contains(&"Lorem ipsum".to_string()));
        assert!(lines.contains(&"Lorem else".to_string()));
    }

    #[test]
    fn query_should_return_line_numbers() {
        let file = NamedTempFile::new().unwrap();
        write(
            file.path(),
            "first line\nLorem ipsum\nthird line\nLorem else\n",
        )
        .unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query(&"Lorem".to_string(), 100).unwrap();
        let line_numbers: Vec<usize> = result
            .into_iter()
            .map(|(line_number, _, _)| line_number)
            .collect();
        assert!(line_numbers.contains(&2));
        assert!(line_numbers.contains(&4));
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
        let result = search.query(&"none".to_string(), 100).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn query_should_return_empty_for_invalid_text() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Lorem ipsum\n").unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        let result = search.query(&"\"".to_string(), 100).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn query_should_match_typo() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Lorem ipsum\n").unwrap();
        let mut search = Search::new().unwrap();
        search.index_lines(file.path().to_path_buf()).unwrap();
        // typo "Lorem" -> "Lroem"
        let result = search.query(&"Lroem".to_string(), 100).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].2, "Lorem ipsum");
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
        let result = search.query(&"apple".to_string(), 25).unwrap();
        assert_eq!(result.len(), 1);
    }
}
