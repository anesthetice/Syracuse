static MATCH_SCORE: i16 = 1;
static MISMATCH_PENALTY: i16 = -1;
static GAP_PENALTY: i16 = -1;

pub fn smith_waterman(seq_1: &str, seq_2: &str) {
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
                matrix[i][j-1] + GAP_PENALTY,
                matrix[i-1][j] + GAP_PENALTY,
                if seq_1[i-1] == seq_2[j-1] {
                    matrix[i-1][j-1] + MATCH_SCORE
                } else {
                    matrix[i-1][j-1] + MISMATCH_PENALTY
                }
            ].into_iter().fold(0_i16, |acc, x| {if x > acc {x} else {acc}});
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
        let n = std::cmp::min(seq_1.len(), seq_2.len()) as i16 * MATCH_SCORE;
        (n*(n+1))/2
    };

    println!("{}", matrix.iter().fold(String::new(), |acc,row| format!("{}{:?}\n", acc, row)));
    println!("total score : {}\nmax score : {}\nratio : {}", total_score, max_score, total_score as f32 / max_score as f32);
}