use crate::config::Config;

pub fn smith_waterman(seq_1: &str, seq_2: &str) -> f64 {
    let match_score: i16 = Config::get().match_score;
    let mismatch_penalty: i16 = Config::get().mismatch_penalty;
    let gap_penalty: i16 = Config::get().gap_penalty;

    // initialization, seq_1 on the left and seq_2 on the top
    let seq_1: Vec<char> = seq_1.chars().collect();
    let seq_2: Vec<char> = seq_2.chars().collect();
    let mut matrix : Vec<Vec<i16>> = vec![vec![0; seq_2.len()+1]; seq_1.len()+1];

    // matrix filling
    let mut total_score: i16 = 0;
    let mut largest_score_index: (usize, usize) = (0,0);
    for i in 1..seq_1.len()+1 { for j in 1..seq_2.len()+1 {
        matrix[i][j] = {
            let score = [
                0,
                matrix[i][j-1] + gap_penalty,
                matrix[i-1][j] + gap_penalty,
                if seq_1[i-1] == seq_2[j-1] {
                    matrix[i-1][j-1] + match_score
                } else {
                    matrix[i-1][j-1] + mismatch_penalty
                },
            ].into_iter().max().unwrap(); // safe unwrap
            if score > total_score {
                total_score = score;
                largest_score_index = (i, j);
            }
            score
        }
    }}

    // traceback
    let (mut i, mut j) = largest_score_index;
    let mut score = total_score;
    while score != 0 {
        score = [
            (i, j-1, matrix[i][j-1]),
            (i-1, j, matrix[i-1][j]),
            (i-1, j-1, matrix[i-1][j-1])
        ].into_iter().fold(0, |acc, (_i, _j, val)| {
            if val > acc {
                i = _i; j = _j;
                val
            } else {acc}
        });
        total_score += score;
    }

    // normalisation
    let max_score = {
        let n = std::cmp::min(seq_1.len(), seq_2.len()) as i16 * match_score;
        (n*(n+1))/2
    };

    total_score as f64 / max_score as f64
}

pub fn needleman_wunsch(seq_1: &str, seq_2: &str) -> f64 {
    let match_score: i16 = Config::get().match_score;
    let mismatch_penalty: i16 = Config::get().mismatch_penalty;
    let gap_penalty: i16 = Config::get().gap_penalty;

    // initialization, seq_1 on the left and seq_2 on the top
    let seq_1: Vec<char> = seq_1.chars().collect();
    let seq_2: Vec<char> = seq_2.chars().collect();
    let mut matrix : Vec<Vec<i16>> = vec![vec![0; seq_2.len()+1]; seq_1.len()+1];

    // matrix filling
    for j in 1..seq_2.len()+1 {matrix[0][j] = j as i16 * gap_penalty}
    for i in 1..seq_1.len()+1 {matrix[i][0] = i as i16 * gap_penalty}
    for i in 1..seq_1.len()+1 { for j in 1..seq_2.len()+1 {
        matrix[i][j] = {
            [
                matrix[i][j-1] + gap_penalty,
                matrix[i-1][j] + gap_penalty,
                if seq_1[i-1] == seq_2[j-1] {
                    matrix[i-1][j-1] + match_score
                } else {
                    matrix[i-1][j-1] + mismatch_penalty
                }
            ].into_iter().max().unwrap() // safe unwrap
         }
    }}

    // traceback
    let (mut i, mut j) = (seq_1.len(), seq_2.len());
    let mut total_score: i16 = matrix[i][j];
    while j != 0 && i != 0 {
        if seq_1[i-1] == seq_2[j-1] {
            i -= 1; j -= 1;
        }
        else {
            // this only considers a single path, gives priority to going 'upwards'
            if matrix[i][j-1] > matrix[i-1][j] {
                j -= 1;
            } else {i-=1}
        }
        total_score += matrix[i][j]
    }
    let i_gp = i as i16 * gap_penalty;
    let j_gp = j as i16 * gap_penalty;
    total_score += i_gp*(i_gp+1)/2 + j_gp*(j_gp+1)/2;

    // normalisation
    if total_score > 0 {
        let max_pos_score = {
            let val = [match_score, mismatch_penalty, gap_penalty].into_iter().max().unwrap().abs();
            let n = std::cmp::max(seq_1.len(), seq_2.len()) as i16 * val;
            (n*(n+1))/2
        };
        total_score as f64 / max_pos_score as f64
    }
    else if total_score == 0 {0.0}
    else {
        let max_neg_score = {
            let val = [match_score, mismatch_penalty, gap_penalty].into_iter().min().unwrap().abs();
            let n = std::cmp::max(seq_1.len(), seq_2.len()) as i16 * val;
            (n*(n+1))/2
        };
        total_score as f64 / max_neg_score as f64
    }
}