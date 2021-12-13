pub fn neighbours<'a, T>(
    map: &'a Vec<Vec<T>>,
    row: usize,
    col: usize,
) -> impl Iterator<Item = ((usize, usize), &'a T)> + 'a {
    if let Some(_) = map.get(row).and_then(|r| r.get(col)) {
        let north = if row > 0 { Some((row - 1, col)) } else { None };
        let west = if col > 0 { Some((row, col - 1)) } else { None };
        let south = if row < map.len() - 1 {
            Some((row + 1, col))
        } else {
            None
        };
        let east = if col < map[0].len() - 1 {
            Some((row, col + 1))
        } else {
            None
        };

        vec![north, west, south, east]
    } else {
        vec![]
    }
    .into_iter()
    .flatten()
    .map(|(nrow, ncol)| ((nrow, ncol), &map[nrow][ncol]))
}

pub fn neighbour_indices_with_diag(
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> Vec<(usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(row_ofs, col_ofs)| ((row as isize + row_ofs), (col as isize + col_ofs)))
    .filter(|(row, col)| 0 <= *row && 0 <= *col)
    .map(|(row, col)| (row as usize, col as usize))
    .filter(|(row, col)| *row < height && *col < width)
    .collect()
}
