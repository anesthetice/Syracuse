#![allow(clippy::needless_range_loop)]

const MATCH_SCORE: i64 = 2;
const MISMATCH_PENALTY: i64 = -1;
const GAP_PENALTY: i64 = -1;

/// Returns a score from 0 to 1 depending on the local alignment of the two string sequences
pub fn smith_waterman(seq_1: &str, seq_2: &str) -> f64 {
    // initialization, seq_1 on the left and seq_2 on the top
    let seq_1: Vec<char> = seq_1.chars().collect();
    let seq_2: Vec<char> = seq_2.chars().collect();
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; seq_2.len() + 1]; seq_1.len() + 1];

    // matrix filling
    let mut total_score: i64 = 0;
    let mut largest_score_index: (usize, usize) = (0, 0);
    for i in 1..seq_1.len() + 1 {
        for j in 1..seq_2.len() + 1 {
            matrix[i][j] = {
                let score = [
                    0,
                    matrix[i][j - 1] + GAP_PENALTY,
                    matrix[i - 1][j] + GAP_PENALTY,
                    if seq_1[i - 1] == seq_2[j - 1] {
                        matrix[i - 1][j - 1] + MATCH_SCORE
                    } else {
                        matrix[i - 1][j - 1] + MISMATCH_PENALTY
                    },
                ]
                .into_iter()
                .max()
                .unwrap(); // safe unwrap
                if score > total_score {
                    total_score = score;
                    largest_score_index = (i, j);
                }
                score
            }
        }
    }

    // traceback
    let (mut i, mut j) = largest_score_index;
    let mut score = total_score;
    while score != 0 {
        score = [
            (i, j - 1, matrix[i][j - 1]),
            (i - 1, j, matrix[i - 1][j]),
            (i - 1, j - 1, matrix[i - 1][j - 1]),
        ]
        .into_iter()
        .fold(0, |acc, (_i, _j, val)| {
            if val > acc {
                i = _i;
                j = _j;
                val
            } else {
                acc
            }
        });
        total_score += score;
    }

    // normalization
    let max_score = {
        let val = [MATCH_SCORE, MISMATCH_PENALTY, GAP_PENALTY]
            .into_iter()
            .max()
            .unwrap()
            .abs();
        let n = std::cmp::min(seq_1.len(), seq_2.len()) as i64 * val;
        (n * (n + 1)) / 2
    };

    total_score as f64 / max_score as f64
}

/// Returns a score from -1 to 1 depending on the global alignment of the two string sequences
pub fn needleman_wunsch(seq_1: &str, seq_2: &str) -> f64 {
    // initialization, seq_1 on the left and seq_2 on the top
    let seq_1: Vec<char> = seq_1.chars().collect();
    let seq_2: Vec<char> = seq_2.chars().collect();
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; seq_2.len() + 1]; seq_1.len() + 1];

    // matrix filling
    for j in 1..seq_2.len() + 1 {
        matrix[0][j] = j as i64 * GAP_PENALTY
    }
    for i in 1..seq_1.len() + 1 {
        matrix[i][0] = i as i64 * GAP_PENALTY
    }
    for i in 1..seq_1.len() + 1 {
        for j in 1..seq_2.len() + 1 {
            matrix[i][j] = {
                [
                    matrix[i][j - 1] + GAP_PENALTY,
                    matrix[i - 1][j] + GAP_PENALTY,
                    if seq_1[i - 1] == seq_2[j - 1] {
                        matrix[i - 1][j - 1] + MATCH_SCORE
                    } else {
                        matrix[i - 1][j - 1] + MISMATCH_PENALTY
                    },
                ]
                .into_iter()
                .max()
                .unwrap() // safe unwrap
            }
        }
    }

    // traceback
    let (mut i, mut j) = (seq_1.len(), seq_2.len());
    let mut total_score: i64 = matrix[i][j];
    while j != 0 && i != 0 {
        if seq_1[i - 1] == seq_2[j - 1] {
            i -= 1;
            j -= 1;
        } else {
            // this only considers a single path, gives priority to going 'upwards'
            if matrix[i][j - 1] > matrix[i - 1][j] {
                j -= 1;
            } else {
                i -= 1
            }
        }
        total_score += matrix[i][j]
    }
    // goes to (0, 0) when reaching an edge
    let i_gp = i as i64 * GAP_PENALTY;
    let j_gp = j as i64 * GAP_PENALTY;
    total_score += i_gp * (i_gp + 1) / 2 + j_gp * (j_gp + 1) / 2;

    // Normalization step, a little over-paranoid, but this should ensure that the result
    // lies between -1 and 1, should maybe scratch it off now that scores are set at compile-time
    if total_score < 0 {
        let max_neg_score = {
            let val = [MATCH_SCORE, MISMATCH_PENALTY, GAP_PENALTY]
                .into_iter()
                .min()
                .unwrap()
                .abs();
            let n = std::cmp::max(seq_1.len(), seq_2.len()) as i64 * val;
            (n * (n + 1)) / 2
        };
        total_score as f64 / max_neg_score as f64
    } else if total_score == 0 {
        0.0
    } else {
        let max_pos_score = {
            let val = [MATCH_SCORE, MISMATCH_PENALTY, GAP_PENALTY]
                .into_iter()
                .max()
                .unwrap()
                .abs();
            let n = std::cmp::max(seq_1.len(), seq_2.len()) as i64 * val;
            (n * (n + 1)) / 2
        };
        total_score as f64 / max_pos_score as f64
    }
}
