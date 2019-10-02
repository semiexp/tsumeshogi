use crate::board::Board;
use crate::common::*;

pub fn solve(board: &Board, max_depth: i32) -> Option<Vec<Move>> {
    assert!(max_depth % 2 == 1);

    let mut steps = vec![];
    solve_first(board, max_depth, &mut steps)
}

fn solve_first(board: &Board, max_depth: i32, steps: &mut Vec<Move>) -> Option<Vec<Move>> {
    let mut moves = board.enumerate_check();

    let mut shortest: Option<Vec<Move>> = None;

    for &mv in &moves {
        let mut board = board.clone();
        board.apply_move(mv);
        steps.push(mv);

        match solve_second(&board, max_depth - 1, steps) {
            None => (),
            Some(mut steps) => {
                let update = match &shortest {
                    None => true,
                    Some(steps2) => steps.len() + 1 < steps2.len(),
                };
                if update {
                    steps.push(mv);
                    shortest = Some(steps);
                }
            }
        }

        steps.pop();
    }

    shortest
}

fn solve_second(board: &Board, max_depth: i32, steps: &mut Vec<Move>) -> Option<Vec<Move>> {
    let mut moves = board.enumerate_check_avoidance();

    if moves.len() == 0 {
        return Some(vec![]);
    }
    if max_depth == 0 {
        return None;
    }

    let mut longest = vec![];

    for &mv in &moves {
        let mut board = board.clone();
        board.apply_move(mv);
        steps.push(mv);

        match solve_first(&board, max_depth - 1, steps) {
            None => {
                steps.pop();
                return None;
            }
            Some(mut steps) => {
                if longest.len() < steps.len() + 1 {
                    steps.push(mv);
                    longest = steps;
                }
            }
        }

        steps.pop();
    }

    Some(longest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::P;

    #[test]
    fn test_problem() {
        {
            let mut board = Board::new();
            board.set_sided_piece(P(0, 4), PIECE_KING.as_second());
            board.set_sided_piece(P(0, 3), PIECE_PAWN.as_second());
            board.set_sided_piece(P(0, 5), PIECE_PAWN.as_second());
            board.set_sided_piece(P(1, 3), PIECE_NIGHT.as_second());
            board.set_sided_piece(P(1, 4), PIECE_NIGHT.as_second());
            board.set_sided_piece(P(1, 5), PIECE_NIGHT.as_second());
            board.set_sided_piece(P(4, 6), PIECE_NIGHT.as_first());

            let sol = solve(&board, 1);
            assert!(sol.is_some() && sol.unwrap().len() == 1);
        }
        {
            let mut board = Board::new();
            board.set_sided_piece(P(0, 7), PIECE_KING.as_second());
            board.set_sided_piece(P(1, 5), PIECE_GOLD.as_first());
            board.set_sided_piece(P(2, 7), PIECE_GOLD.as_first());

            let sol = solve(&board, 3);
            assert!(sol.is_some() && sol.unwrap().len() == 3);
        }
    }
}
