extern crate anyhow;

mod test_utils;
use rust_tokenizers::tokenizer::{
    MultiThreadedTokenizer, T5Tokenizer, Tokenizer, TruncationStrategy,
};
use rust_tokenizers::{Offset, TokenizedInput};
use test_utils::download_file_to_cache;

#[test]
fn test_t5_tokenization() -> anyhow::Result<()> {
    let vocab_path = download_file_to_cache(
        "https://s3.amazonaws.com/models.huggingface.co/bert/t5-spiece.model",
        "t5-spiece.model",
    )
    .unwrap();

    let t5_tokenizer = T5Tokenizer::from_file(vocab_path.to_str().unwrap(), false)?;

    let original_strings = [
        "…",
        "This is a sample sentence to be tokénized",
        "Wondering how this will get tokenized 🤔 ?",
        "İs th!s 𩸽 Ϻ Šœ Ugljšić dấu nặng",
        "   İs th!s    𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
        " � İs th!s �� 𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
    ];

    let expected_results = [
        TokenizedInput {
            token_ids: vec![3, 233, 1],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 0, end: 1 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![100, 19, 3, 9, 3106, 7142, 12, 36, 12, 157, 154, 29, 1601, 1],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 4 }),
                Some(Offset { begin: 4, end: 7 }),
                Some(Offset { begin: 7, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 16 }),
                Some(Offset { begin: 16, end: 25 }),
                Some(Offset { begin: 25, end: 28 }),
                Some(Offset { begin: 28, end: 31 }),
                Some(Offset { begin: 31, end: 34 }),
                Some(Offset { begin: 34, end: 35 }),
                Some(Offset { begin: 35, end: 36 }),
                Some(Offset { begin: 37, end: 38 }),
                Some(Offset { begin: 38, end: 42 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![16347, 53, 149, 48, 56, 129, 14145, 1601, 3, 2, 3, 58, 1],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 6 }),
                Some(Offset { begin: 6, end: 9 }),
                Some(Offset { begin: 9, end: 13 }),
                Some(Offset { begin: 13, end: 18 }),
                Some(Offset { begin: 18, end: 23 }),
                Some(Offset { begin: 23, end: 27 }),
                Some(Offset { begin: 27, end: 33 }),
                Some(Offset { begin: 33, end: 37 }),
                Some(Offset { begin: 37, end: 38 }),
                Some(Offset { begin: 38, end: 39 }),
                Some(Offset { begin: 39, end: 40 }),
                Some(Offset { begin: 40, end: 41 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                3, 2, 7, 3, 189, 55, 7, 3, 2, 3, 2, 3, 2, 412, 122, 40, 354, 2, 23, 2, 3, 26, 2,
                76, 3, 29, 2, 1725, 1,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 1, end: 2 }),
                Some(Offset { begin: 2, end: 3 }),
                Some(Offset { begin: 3, end: 5 }),
                Some(Offset { begin: 5, end: 6 }),
                Some(Offset { begin: 6, end: 7 }),
                Some(Offset { begin: 7, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 11, end: 12 }),
                Some(Offset { begin: 12, end: 14 }),
                Some(Offset { begin: 14, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 18 }),
                Some(Offset { begin: 18, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 23 }),
                Some(Offset { begin: 23, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 28 }),
                Some(Offset { begin: 28, end: 29 }),
                Some(Offset { begin: 29, end: 31 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                3, 3, 3, 2, 7, 3, 189, 55, 7, 3, 3, 3, 3, 2, 3, 2, 3, 2, 3, 3, 412, 122, 40, 354,
                2, 23, 2, 3, 3, 26, 2, 76, 3, 29, 2, 1725, 3, 3, 3, 3, 3, 1,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 1, end: 2 }),
                Some(Offset { begin: 2, end: 3 }),
                Some(Offset { begin: 3, end: 4 }),
                Some(Offset { begin: 4, end: 5 }),
                Some(Offset { begin: 5, end: 6 }),
                Some(Offset { begin: 6, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 11, end: 12 }),
                Some(Offset { begin: 12, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 18 }),
                Some(Offset { begin: 18, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 28 }),
                Some(Offset { begin: 28, end: 29 }),
                Some(Offset { begin: 29, end: 30 }),
                Some(Offset { begin: 30, end: 31 }),
                Some(Offset { begin: 31, end: 32 }),
                Some(Offset { begin: 32, end: 33 }),
                Some(Offset { begin: 33, end: 34 }),
                Some(Offset { begin: 34, end: 35 }),
                Some(Offset { begin: 35, end: 36 }),
                Some(Offset { begin: 36, end: 37 }),
                Some(Offset { begin: 37, end: 38 }),
                Some(Offset { begin: 38, end: 40 }),
                Some(Offset { begin: 40, end: 41 }),
                Some(Offset { begin: 41, end: 42 }),
                Some(Offset { begin: 42, end: 43 }),
                Some(Offset { begin: 43, end: 44 }),
                Some(Offset { begin: 44, end: 45 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                3, 3, 2, 7, 3, 189, 55, 7, 3, 3, 2, 3, 2, 3, 2, 3, 3, 412, 122, 40, 354, 2, 23, 2,
                3, 3, 26, 2, 76, 3, 29, 2, 1725, 3, 3, 3, 3, 3, 1,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 2, end: 3 }),
                Some(Offset { begin: 3, end: 4 }),
                Some(Offset { begin: 4, end: 5 }),
                Some(Offset { begin: 5, end: 6 }),
                Some(Offset { begin: 6, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 18 }),
                Some(Offset { begin: 18, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 28 }),
                Some(Offset { begin: 28, end: 29 }),
                Some(Offset { begin: 29, end: 30 }),
                Some(Offset { begin: 30, end: 31 }),
                Some(Offset { begin: 31, end: 32 }),
                Some(Offset { begin: 32, end: 33 }),
                Some(Offset { begin: 33, end: 34 }),
                Some(Offset { begin: 34, end: 35 }),
                Some(Offset { begin: 35, end: 36 }),
                Some(Offset { begin: 36, end: 37 }),
                Some(Offset { begin: 37, end: 38 }),
                Some(Offset { begin: 38, end: 40 }),
                Some(Offset { begin: 40, end: 41 }),
                Some(Offset { begin: 41, end: 42 }),
                Some(Offset { begin: 42, end: 43 }),
                Some(Offset { begin: 43, end: 44 }),
                Some(Offset { begin: 44, end: 45 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
    ]
    .to_vec();

    let output = MultiThreadedTokenizer::encode_list(
        &t5_tokenizer,
        &original_strings,
        128,
        &TruncationStrategy::LongestFirst,
        0,
    );

    for (_idx, (predicted, expected)) in output.iter().zip(expected_results.iter()).enumerate() {
        let original_sentence_chars: Vec<char> = original_strings[_idx].chars().collect();
        for (idx, offset) in predicted.token_offsets.iter().enumerate() {
            match offset {
                Some(offset) => {
                    let (start_char, end_char) = (offset.begin as usize, offset.end as usize);
                    let text: String = original_sentence_chars[start_char..end_char]
                        .iter()
                        .collect();
                    println!(
                        "{:<2?} | {:<10} | {:<10} | {:<10?}",
                        offset,
                        text,
                        t5_tokenizer.decode(&[predicted.token_ids[idx]], false, false),
                        predicted.mask[idx]
                    )
                }
                None => continue,
            }
        }

        assert_eq!(predicted.token_ids, expected.token_ids);
        assert_eq!(predicted.token_offsets, expected.token_offsets);
    }

    Ok(())
}
