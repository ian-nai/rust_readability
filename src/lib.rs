use wordsworth::syllable_counter;

use punkt::params::Standard;
use punkt::SentenceTokenizer;
use punkt::Trainer;
use punkt::TrainingData;

use unicode_segmentation::UnicodeSegmentation;

use regex::Regex;

use std::fs;

pub fn word_list_from_string(string_to_analyze: &str) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();
    let words = string_to_analyze.unicode_words();
    for w in words {
        word_list.push(w.to_string());
    }

    word_list
}

pub fn word_list_from_file(filename: &str) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();
    let string_from_file = fs::read_to_string(filename).expect("Unable to read file");
    let words = string_from_file.unicode_words();
    for w in words {
        word_list.push(w.to_string());
    }
    for s in &word_list {
        println!("{}", s);
    }
    word_list
}

pub fn file_to_string(filename: &str) -> String {
    let string_from_file = fs::read_to_string(filename).expect("Unable to read file");
    string_from_file
}

pub fn long_words(word_list: Vec<String>) -> f64 {
    let mut sum = 0;
    for word in word_list {
        if word.len() >= 6 {
            sum += 1;
        }
    }
    let long_words_count: f64;
    long_words_count = sum as f64;
    long_words_count
}

pub fn percent_long_words(word_list: Vec<String>) -> f64 {
    let list_count = word_list.len() as i32;
    let long_words_count = long_words(word_list);
    let percent_longwords = long_words_count as f64 / list_count as f64;
    percent_longwords
}

pub fn character_count(string_to_analyze: &str) -> f64 {
    let re = Regex::new(r"[^\w]").unwrap();
    let result = re.replace_all(string_to_analyze, "");

    let character_count: f64;
    character_count = result.graphemes(true).count() as f64;

    character_count
}

pub fn linsear_scoring(sylcount_list: Vec<i32>) -> f64{
    let mut score_sum: f64;
    score_sum = 0.0;
    for syl in &sylcount_list {
        if (*syl as f64) < 2.5 {
            score_sum += 1.0;
        } else {
            score_sum += 3.0;
        }
    }
    score_sum
}

pub fn syllable_count_list(word_list: Vec<String>) -> Vec<i32> {
    let mut sylcount_list: Vec<i32> = Vec::new();
    for word in &word_list {
        sylcount_list.push(syllable_counter(&word).try_into().unwrap());
    }
    sylcount_list
}

pub fn avg_syl_count(sylcount_list: Vec<i32>) -> f64 {
    let mut sum = 0;
    for w in &sylcount_list {
        sum += w;
    }
    let avg_syls: f64;
    avg_syls = sum as f64 / sylcount_list.len() as f64;
    avg_syls
}

// num of words with 2 syllables or less;
pub fn short_syl_count(word_list: Vec<String>) -> i32 {
    let sylcount_list = syllable_count_list(word_list);
    let mut short_syls = 0;
    for syl in sylcount_list {
        if syl <= 2 {
            short_syls += 1;
        }
    }
    short_syls
}

// num of words with 3 syls or more;
pub fn long_syl_count(word_list: Vec<String>) -> i32 {
    let sylcount_list = syllable_count_list(word_list);
    let mut long_syls = 0;
    for syl in sylcount_list {
        if syl >= 3 {
            long_syls += 1;
        }
    }
    long_syls
}

pub fn wordcount_list(word_list: Vec<String>) -> i32 {
    let wordcount: i32;
    wordcount = word_list.len() as i32;
    wordcount
}

pub fn split_into_sentences(doc: &str) -> Vec<String> {
    let trainer: Trainer<Standard> = Trainer::new();
    let mut data = TrainingData::new();

    trainer.train(doc, &mut data);
    let mut sent_list: Vec<String> = Vec::new();

    for s in SentenceTokenizer::<Standard>::new(doc, &data) {
        sent_list.push(s.to_string());
    }
    sent_list
}

pub fn sent_word_counts(doc: &str) -> Vec<usize> {
    let trainer: Trainer<Standard> = Trainer::new();
    let mut data = TrainingData::new();

    trainer.train(doc, &mut data);
    let mut sent_word_list: Vec<String> = Vec::new();

    let mut sent_wordcount_list: Vec<usize> = Vec::new();

    for s in SentenceTokenizer::<Standard>::new(doc, &data) {
        sent_word_list.push(s.to_string());
    }
    for s in sent_word_list {
        let mut temp_vec: Vec<String> = Vec::new();
        for word in s.split_whitespace() {
            temp_vec.push(word.to_string());
        }
        sent_wordcount_list.push(temp_vec.len());
    }
    sent_wordcount_list
}

pub fn sent_avg_wordcount(sent_wordcount_list: Vec<usize>) -> f64 {
    let mut sum = 0;
    for w in &sent_wordcount_list {
        sum += w;
    }
    let avg_word_length: f64;
    avg_word_length = sum as f64 / sent_wordcount_list.len() as f64;
    avg_word_length
}

pub fn lix(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    //total_words
    let all_words = word_list_from_string(&string_to_analyze);
    //num long words
    let num_long_words = percent_long_words(all_words);
    //avg_num_words/sentence
    let sent_wordcount_list = sent_word_counts(&string_to_analyze);
    let avg_words_per_sentence = sent_avg_wordcount(sent_wordcount_list);
    let lix_index = num_long_words + avg_words_per_sentence;
    println!("{}", lix_index);
    lix_index
}

pub fn rix(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    //total_words
    let all_words = word_list_from_string(&string_to_analyze);
    //num long words
    let num_long_words = long_words(all_words);
    let num_of_sents = split_into_sentences(&string_to_analyze).len() as f64;
    let rix_index = num_long_words / num_of_sents;
    println!("{}", rix_index);
    rix_index
}

pub fn flesch(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    let sent_counts = sent_word_counts(&string_to_analyze);
    let avg_sent_length = sent_avg_wordcount(sent_counts);
    //syls
    let all_words = word_list_from_string(&string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    let flesch_index = 206.835 - (1.015 * avg_sent_length) - (84.6 * avg_syls);
    println!("{}", flesch_index);
    flesch_index
}

pub fn flesch_kincaid(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    // avg_words_per_sentence
    let sent_counts = sent_word_counts(&string_to_analyze);
    let avg_sent_length = sent_avg_wordcount(sent_counts);
    // avg syls per word
    let all_words = word_list_from_string(&string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    let flesch_kincaid_index = (0.39 * avg_sent_length) + (11.8 * avg_syls) - 15.59;
    println!("{}", flesch_kincaid_index);
    flesch_kincaid_index
}

pub fn linsear_write(file_to_analyze: &str) -> f64{
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    //compiling variables for linsear_write 
    let all_words = word_list_from_string(&string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let summed_score = linsear_scoring(all_syls);
    let sent_count = split_into_sentences(&string_to_analyze).len() as f64;
    // compute the score
    let provisional_result = summed_score / sent_count;
    // provisional adjustment 
    let linsear_write_score: f64;
    if provisional_result < 20.0 {
        linsear_write_score = (provisional_result / 2.0) - 1.0
    } else {
        linsear_write_score = provisional_result / 2.0
    }
    println!("{}", linsear_write_score);
    linsear_write_score
}

pub fn coleman_liau(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    //total_words
    let all_words = word_list_from_string(&string_to_analyze).len() as f64;
    // characters
    let chars = character_count(&string_to_analyze);
    // sents
    let sent_count = split_into_sentences(&string_to_analyze).len() as f64;

    let coleman_liau_index =
        0.0588 * ((chars / all_words) * 100.0) - 0.296 * ((sent_count / chars) * 100.0) - 15.8;
    println!("{}", coleman_liau_index);
    coleman_liau_index
}

pub fn ari(file_to_analyze: &str) -> f64 {
    //file to string
    let string_to_analyze = file_to_string(file_to_analyze);
    //total_words
    let all_words = word_list_from_string(&string_to_analyze).len() as f64;
    // characters
    let chars = character_count(&string_to_analyze);
    // sents
    let sent_count = split_into_sentences(&string_to_analyze).len() as f64;

    let automated_readability_index =
        (4.71 * (chars / all_words)) + (0.5 * (all_words / sent_count)) - 21.43;
    println!("{}", automated_readability_index);
    automated_readability_index
}

pub fn lix_string(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze);
    //num long words
    let num_long_words = percent_long_words(all_words);
    //avg_num_words/sentence
    let sent_wordcount_list = sent_word_counts(string_to_analyze);
    for x in &sent_wordcount_list {
        println!("{}", x);
    }
    let avg_words_per_sentence = sent_avg_wordcount(sent_wordcount_list);
    let lix_index = num_long_words + avg_words_per_sentence;
    println!("{}", lix_index);
    lix_index
}

pub fn rix_string(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze);
    //num long words
    let num_long_words = long_words(all_words);
    // let sentences
    let num_of_sents = split_into_sentences(string_to_analyze).len() as f64;
    let rix_index = num_long_words / num_of_sents;
    println!("{}", rix_index);
    rix_index
}

pub fn flesch_string(string_to_analyze: &str) -> f64 {
    let sent_counts = sent_word_counts(string_to_analyze);
    let avg_sent_length = sent_avg_wordcount(sent_counts);
    //syls
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    let flesch_index = 206.835 - (1.015 * avg_sent_length) - (84.6 * avg_syls);
    println!("{}", flesch_index);
    flesch_index
}

pub fn flesch_kincaid_string(string_to_analyze: &str) -> f64 {
    // avg_words_per_sentence
    let sent_counts = sent_word_counts(string_to_analyze);
    let avg_sent_length = sent_avg_wordcount(sent_counts);
    // avg syls per word
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    let flesch_kincaid_index = (0.39 * avg_sent_length) + (11.8 * avg_syls) - 15.59;
    println!("{}", flesch_kincaid_index);
    flesch_kincaid_index
}

pub fn linsear_write_string(string_to_analyze: &str) -> f64{
    //compiling variables for linsear_write 
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let summed_score = linsear_scoring(all_syls);
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;
    // compute the score
    let provisional_result = summed_score / sent_count;
    // provisional adjustment 
    let linsear_write_score: f64;
    if provisional_result < 20.0 {
        linsear_write_score = (provisional_result / 2.0) - 1.0
    } else {
        linsear_write_score = provisional_result / 2.0
    }
    println!("{}", linsear_write_score);
    linsear_write_score
}

pub fn coleman_liau_string(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    let coleman_liau_index =
        0.0588 * ((chars / all_words) * 100.0) - 0.296 * ((sent_count / chars) * 100.0) - 15.8;
    println!("{}", coleman_liau_index);
    coleman_liau_index
}

pub fn automated_readability_index_string(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    let automated_readability_index =
        (4.71 * (chars / all_words)) + (0.5 * (all_words / sent_count)) - 21.43;
    println!("{}", automated_readability_index);
    automated_readability_index
}
