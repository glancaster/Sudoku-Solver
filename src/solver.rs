use iced::{
    futures::{SinkExt, Stream},
    stream::try_channel,
};

pub fn brute_force(
    mut board: [[usize; 9]; 9],
    puzzle: [[usize; 9]; 9],
) -> impl Stream<Item = Result<[[usize; 9]; 9], ()>> {
    // try_channel: Creates a new Stream that produces the items sent from a Future
    // that can fail to the mpsc::Sender provided to the closure.
    try_channel(1, move |mut o| async move {
        let mut i = 0;
        let mut dir = 1;
        while i < 81 {
            let row = i / 9 as usize;
            let col = i % 9 as usize;
            let cell_row = row / 3 as usize;
            let cell_col = col / 3 as usize;
            //println!("element {i} cell row {cell_row} cell col {cell_col}");

            if puzzle[row][col] == 0 {
                loop {
                    // tokio::time::sleep(Duration::from_millis(200)).await;
                    board[row][col] += 1;
                    let digit = board[row][col];
                    //println!("{digit}");
                    if digit == 10 {
                        board[row][col] = 0;
                        i -= 1;
                        dir = -1;
                        break;
                    }
                    let col_check = board
                        .iter()
                        .map(|r| r[col])
                        .filter(|&e| e == board[row][col])
                        .collect::<Vec<_>>()
                        .len();
                    let row_check = board[row]
                        .iter()
                        .filter(|&e| e == &board[row][col])
                        .collect::<Vec<_>>()
                        .len();
                    let cr = cell_row * 3;
                    let cc = cell_col * 3;
                    let cell_check = board[cr..cr + 3]
                        .iter()
                        .flat_map(|r| r[cc..cc + 3].iter().collect::<Vec<_>>())
                        .filter(|&e| e == &board[row][col])
                        .collect::<Vec<_>>()
                        .len();
                    //println!("COL {:?} {}", b.iter().map(|r| r[col]).collect::<Vec<_>>(), col_check);
                    //println!("ROW {:?} {}", b[row], row_check);
                    //println!("CELL {:?} {}", b[cr..cr+3].iter().flat_map(|r| r[cc..cc+3].iter().collect::<Vec<_>>()).collect::<Vec<_>>(), cell_check);

                    if row_check == 1 && col_check == 1 && cell_check == 1 {
                        i += 1;
                        dir = 1;
                        break;
                    }

                    let _ = o.send(board).await;
                }
            } else {
                if i == 0 {
                    dir = 1;
                }
                if dir == 1 {
                    i += 1;
                } else if dir == -1 {
                    i -= 1;
                }
            }
        }
        println!("Solved?");

        let _ = o.send(board).await;

        Ok(())
    })
}
