pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = minefield.first().unwrap().len();

    let mut mines = vec![vec![0; width]; height];

    for (i, &row) in minefield.iter().enumerate() {
        for (j, cell) in row.char_indices() {
            if cell == '*' {
                mines[i][j] = -1;

                if i != 0 && j != 0 && minefield[i - 1][j - 1] != '*' {
                    mines[i - 1][j - 1] += 1;
                }
                if i != 0 && minefield[i - 1][j] != '*'{
                    mines[i - 1][j] += 1;
                }
                if i != 0 && j != width - 1 && minefield[i - 1][j + 1] != '*'{
                    mines[i - 1][j + 1] += 1;
                }
                if j != 0 && minefield[i][j - 1] != '*'{
                    mines[i][j - 1] += 1;
                }
                if j != width - 1 && minefield[i][j + 1] != '*'{
                    mines[i][j + 1] += 1;
                }
                if i != height && j != 0 && minefield[i + 1][j - 1] != '*'{
                    mines[i + 1][j - 1] += 1;
                }
                if i != height && minefield[i + 1][j] != '*'{
                    mines[i + 1][j] += 1;
                }
                if i != 0 && j != height - 1 && minefield[i + 1][j + 1] != '*' {
                    mines[i + 1][j + 1] += 1;
                }
            }
        }
    };

    mines
        .iter()
        .map(|row|
            row.iter().map(
                |&c| match c {
                    -1 => "*".to_string(),
                    0 => " ".to_string(),
                    v => v.to_string(),
                }).collect())
        .collect()
}
