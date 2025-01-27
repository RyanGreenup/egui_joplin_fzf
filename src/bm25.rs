use crate::note::Note;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

fn default_tokenize(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

pub fn bm25_trigram(documents: &[String], target_string: &str) -> Vec<String> {
    bm25(documents, target_string, ngram_tokenize)
}

/// Sort notes by relevance to query using BM25 with trigram tokenization
#[allow(dead_code)]
fn sort_notes_trigram(notes: &[Note], query: &str) -> Vec<Note> {
    // Extract document strings from notes
    let documents: Vec<String> = notes
        .iter()
        .map(|note| format!("{} {}", note.title, note.body))
        .collect();

    // Get sorted indices using BM25 trigram
    let sorted_docs = bm25_trigram(&documents, query);

    // Map sorted documents back to original notes
    sorted_docs
        .into_iter()
        .filter_map(|doc| {
            notes
                .iter()
                .find(|note| format!("{} {}", note.title, note.body) == doc)
                .cloned()
        })
        .collect()
}

pub fn bm25(
    documents: &[String],
    target_string: &str,
    tokenize: fn(&str) -> Vec<String>,
) -> Vec<String> {
    // Tokenize documents
    let tokenized_docs: Vec<Vec<String>> = documents.iter().map(|doc| tokenize(doc)).collect();

    // Process query terms
    let query_terms = tokenize(target_string);
    let term_frequencies_query: HashMap<String, usize> = {
        let unique_terms: HashSet<_> = query_terms.iter().cloned().collect();
        unique_terms
            .into_iter()
            .map(|term| {
                let count = query_terms.iter().filter(|&t| t == &term).count();
                (term, count)
            })
            .collect()
    };

    // Calculate average document length
    let total_length: usize = tokenized_docs.iter().map(|doc| doc.len()).sum();
    let avgdl = total_length as f64 / tokenized_docs.len() as f64;

    // Set IDF values (simplified to 1.0)
    let idf_values: HashMap<String, f64> = term_frequencies_query
        .keys()
        .map(|term| (term.clone(), 1.0))
        .collect();

    // BM25 parameters
    let k1 = 1.5;
    let b = 0.75;

    // Calculate scores
    let mut bm25_scores: HashMap<usize, f64> = HashMap::new();

    for (i, doc) in tokenized_docs.iter().enumerate() {
        let term_frequencies_doc: HashMap<String, usize> = {
            let unique_terms: HashSet<_> = doc.iter().cloned().collect();
            unique_terms
                .into_iter()
                .map(|term| {
                    let count = doc.iter().filter(|&t| t == &term).count();
                    (term, count)
                })
                .collect()
        };

        let mut score = 0.0;

        for query_term in term_frequencies_query.keys() {
            if let Some(&tf) = term_frequencies_doc.get(query_term) {
                let idf = idf_values.get(query_term).unwrap_or(&1.0);
                let numerator = idf * (tf as f64 * (k1 + 1.0));
                let denominator = tf as f64 + k1 * (1.0 - b + b * (doc.len() as f64 / avgdl));
                score += numerator / denominator;
            }
        }

        bm25_scores.insert(i, score);
    }

    // Sort by scores
    let mut indices: Vec<usize> = (0..documents.len()).collect();
    indices.sort_by_key(|&i| OrderedFloat(-bm25_scores.get(&i).unwrap_or(&0.0)));

    // Return sorted documents
    indices.into_iter().map(|i| documents[i].clone()).collect()
}

fn ngram_tokenize(s: &str) -> Vec<String> {
    const N: usize = 3; // trigrams

    // Handle edge cases
    if s.len() < N {
        return vec![s.to_string()];
    }

    // Convert to lowercase to make matching case-insensitive
    let text = s.to_lowercase();

    // Create sliding window of N characters
    text.chars()
        .collect::<Vec<char>>()
        .windows(N)
        .map(|window| window.iter().collect::<String>())
        .collect()
}

#[allow(dead_code)]
fn main() {
    let documents = vec![
        "the quick brown fox jumps over the lazy dog".to_string(),
        "never jump over the lazy dog quickly".to_string(),
        "a quick movement of the enemy will jeopardize six gunboats".to_string(),
        "quick and nimble cats jump over sleeping dogs".to_string(),
    ];
    let target_string = "never";
    let sorted_documents = bm25(&documents, target_string, default_tokenize);
    println!("{:?}", sorted_documents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let documents = vec![
            "the quick brown fox".to_string(),
            "never give up".to_string(),
            "quick brown".to_string(),
        ];
        let target = "never";
        let result = bm25(&documents, target, default_tokenize);
        assert_eq!(result[0], "never give up");
    }

    #[test]
    fn test_empty_query() {
        let documents = vec![
            "the quick brown fox".to_string(),
            "never give up".to_string(),
        ];
        let target = "";
        let result = bm25(&documents, target, default_tokenize);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_no_matches() {
        let documents = vec![
            "the quick brown fox".to_string(),
            "never give up".to_string(),
        ];
        let target = "zzzzz";
        let result = bm25(&documents, target, default_tokenize);
        assert_eq!(result.len(), 2);
        // Documents should still be returned, just with zero scores
    }

    #[test]
    fn test_custom_tokenizer() {
        let documents = vec![
            "the-quick-brown-fox".to_string(),
            "never-give-up".to_string(),
        ];
        let custom_tokenize = |s: &str| s.split('-').map(String::from).collect();
        let target = "never";
        let result = bm25(&documents, target, custom_tokenize);
        assert_eq!(result[0], "never-give-up");
    }

    #[test]
    fn test_ngram_tokenizer() {
        let documents = vec![
            "hello world".to_string(),
            "help me".to_string(),
            "world peace".to_string(),
        ];

        // This will test trigrams (n=3)
        // For example, "hello" should become: ["hel", "ell", "llo"]
        let target = "hell";
        let result = bm25(&documents, target, ngram_tokenize);

        // "hello world" and "help me" should be ranked higher than "world peace"
        // as they both contain "hel" trigrams
        assert_eq!(result[0], "hello world");
        assert_eq!(result[1], "help me");
        assert_eq!(result[2], "world peace");
    }

    #[test]
    fn test_ngram_tokenize_basic() {
        let result = ngram_tokenize("hello");
        let expected = vec!["hel", "ell", "llo"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ngram_tokenize_short_input() {
        let result = ngram_tokenize("hi");
        let expected = vec!["hi"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ngram_tokenize_case_insensitive() {
        let result = ngram_tokenize("Hello");
        let expected = vec!["hel", "ell", "llo"];
        assert_eq!(result, expected);
    }
}
