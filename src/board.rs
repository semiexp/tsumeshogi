use crate::common::*;
use crate::{D, P};
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Board {
    pieces: [SidedPiece; BOARD_CELLS as usize],
    hand_first: [i8; PIECE_TYPES],
    hand_second: [i8; PIECE_TYPES],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [EMPTY_CELL; BOARD_CELLS as usize],
            hand_first: [0; PIECE_TYPES],
            hand_second: [0; PIECE_TYPES],
        }
    }
    pub fn is_inside_board(&self, pos: P) -> bool {
        0 <= pos.0 && pos.0 < BOARD_SIZE as i32 && 0 <= pos.1 && pos.1 < BOARD_SIZE as i32
    }
    pub fn get_sided_piece(&self, pos: P) -> SidedPiece {
        assert!(self.is_inside_board(pos));
        self.pieces[(pos.0 * 9 + pos.1) as usize]
    }
    pub fn set_sided_piece(&mut self, pos: P, piece: SidedPiece) {
        assert!(self.is_inside_board(pos));
        self.pieces[(pos.0 * 9 + pos.1) as usize] = piece;
    }
    pub fn set_first_hand(&mut self, piece: Piece, n: i8) {
        self.hand_first[piece.0 as usize] = n;
    }
    pub fn get_first_hand(&mut self, piece: Piece) -> i8 {
        self.hand_first[piece.0 as usize]
    }
    pub fn set_second_hand(&mut self, piece: Piece, n: i8) {
        self.hand_second[piece.0 as usize] = n;
    }
    pub fn get_second_hand(&mut self, piece: Piece) -> i8 {
        self.hand_second[piece.0 as usize]
    }
    pub fn locate_second_king(&self) -> P {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.get_sided_piece(P(y, x)) == SECOND_KING {
                    return P(y, x);
                }
            }
        }
        panic!();
    }
    pub fn is_check(&self) -> bool {
        let king_pos = self.locate_second_king();

        for i in 0..(PIECE_TYPES * 2) {
            for j in 0..PIECE_MOVES_COUNT {
                if PIECE_MOVES[i][j] == D(0, 0) {
                    break;
                }
                let pos = king_pos + PIECE_MOVES[i][j].flip();
                if !self.is_inside_board(pos) {
                    continue;
                }
                if self.get_sided_piece(pos) == SidedPiece(i as i8) {
                    return true;
                }
            }
        }
        if self.find_check_by_ranging_moves(king_pos, D(1, 0), PIECE_LANCE, PIECE_LANCE)
            || self.find_check_by_ranging_moves(
                king_pos,
                D(-1, -1),
                PIECE_BISHOP,
                PIECE_BISHOP.promote(),
            )
            || self.find_check_by_ranging_moves(
                king_pos,
                D(-1, 1),
                PIECE_BISHOP,
                PIECE_BISHOP.promote(),
            )
            || self.find_check_by_ranging_moves(
                king_pos,
                D(1, -1),
                PIECE_BISHOP,
                PIECE_BISHOP.promote(),
            )
            || self.find_check_by_ranging_moves(
                king_pos,
                D(1, 1),
                PIECE_BISHOP,
                PIECE_BISHOP.promote(),
            )
            || self.find_check_by_ranging_moves(
                king_pos,
                D(-1, 0),
                PIECE_ROOK,
                PIECE_ROOK.promote(),
            )
            || self.find_check_by_ranging_moves(
                king_pos,
                D(0, -1),
                PIECE_ROOK,
                PIECE_ROOK.promote(),
            )
            || self.find_check_by_ranging_moves(king_pos, D(0, 1), PIECE_ROOK, PIECE_ROOK.promote())
            || self.find_check_by_ranging_moves(king_pos, D(1, 0), PIECE_ROOK, PIECE_ROOK.promote())
        {
            return true;
        }
        false
    }
    fn find_check_by_ranging_moves(&self, pos: P, dir: D, target: Piece, target2: Piece) -> bool {
        let target = target.as_first();
        let target2 = target2.as_first();
        for i in 1..BOARD_SIZE {
            let pos2 = pos + dir * i;
            if !self.is_inside_board(pos2) {
                break;
            }
            let piece = self.get_sided_piece(pos2);
            if piece == target || piece == target2 {
                return true;
            }
            if !piece.is_empty() {
                return false;
            }
        }
        false
    }
    pub fn apply_move(&mut self, mv: Move) {
        // TODO: capturing pieces
        match mv {
            Move::NoPromotion(src, dest) => {
                self.set_sided_piece(dest, self.get_sided_piece(src));
                self.set_sided_piece(src, EMPTY_CELL);
            }
            Move::Promotion(src, dest) => {
                self.set_sided_piece(dest, self.get_sided_piece(src).promote());
                self.set_sided_piece(src, EMPTY_CELL);
            }
            Move::FromHand(pos, piece) => {
                if piece.is_first() {
                    self.hand_first[piece.0 as usize] -= 1;
                } else {
                    self.hand_second[piece.to_piece().0 as usize] -= 1;
                }
                self.set_sided_piece(pos, piece);
            }
        }
    }
    pub fn is_promotion_zone(pos: P, is_second: bool) -> bool {
        if is_second {
            pos.0 >= 7
        } else {
            pos.0 <= 2
        }
    }
    pub fn has_further_move(pos: P, piece: Piece, is_second: bool) -> bool {
        if piece == PIECE_PAWN || piece == PIECE_LANCE {
            if (!is_second && pos.0 == 0) || (is_second && pos.0 == 8) {
                return false;
            }
        } else if piece == PIECE_NIGHT {
            if (!is_second && pos.0 <= 1) || (is_second && pos.0 >= 7) {
                return false;
            }
        }
        true
    }
    pub fn enumerate_moves(&self, pos: P) -> Vec<Move> {
        let mut ret = vec![];
        let piece = self.get_sided_piece(pos);
        if piece.is_empty() {
            return ret;
        }
        let is_second = piece.is_second();
        let piece = piece.to_piece();
        let piece_id = piece.0 as usize;

        for i in 0..PIECE_MOVES_COUNT {
            if PIECE_MOVES[piece_id][i] == D(0, 0) {
                break;
            }

            let pos2 = pos + PIECE_MOVES[piece_id][i].flip_if(is_second);
            if !self.is_inside_board(pos2) {
                continue;
            }
            let piece2 = self.get_sided_piece(pos2);
            if !((!is_second && piece2.is_first()) || (is_second && piece2.is_second())) {
                if piece.has_promotion()
                    && (Board::is_promotion_zone(pos, is_second)
                        || Board::is_promotion_zone(pos2, is_second))
                {
                    ret.push(Move::Promotion(pos, pos2));
                }
                if Board::has_further_move(pos, piece, is_second) {
                    ret.push(Move::NoPromotion(pos, pos2));
                }
            }
        }

        if piece == PIECE_LANCE {
            self.enumerate_ranging_moves(
                pos,
                D(-1, 0).flip_if(is_second),
                is_second,
                piece,
                &mut ret,
            );
        }
        if piece.capture() == PIECE_BISHOP {
            self.enumerate_ranging_moves(pos, D(-1, -1), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(-1, 1), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(1, -1), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(1, 1), is_second, piece, &mut ret);
        }
        if piece.capture() == PIECE_ROOK {
            self.enumerate_ranging_moves(pos, D(-1, 0), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(0, -1), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(0, 1), is_second, piece, &mut ret);
            self.enumerate_ranging_moves(pos, D(1, 0), is_second, piece, &mut ret);
        }
        ret
    }
    fn enumerate_ranging_moves(
        &self,
        pos: P,
        dir: D,
        is_second: bool,
        piece: Piece,
        dest: &mut Vec<Move>,
    ) {
        for i in 1..BOARD_SIZE {
            let pos2 = pos + dir * i;
            if !self.is_inside_board(pos2) {
                break;
            }
            let piece2 = self.get_sided_piece(pos2);
            if (!is_second && piece2.is_first()) || (is_second && piece2.is_second()) {
                break;
            }
            if piece.has_promotion()
                && (Board::is_promotion_zone(pos, is_second)
                    || Board::is_promotion_zone(pos2, is_second))
            {
                dest.push(Move::Promotion(pos, pos2));
            }
            if Board::has_further_move(pos, piece, is_second) {
                dest.push(Move::NoPromotion(pos, pos2));
            }
            if !piece2.is_empty() {
                break;
            }
        }
    }
    pub fn enumerate_check(&self) -> Vec<Move> {
        // TODO: more efficient algorithm
        let mut ret = vec![];
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let piece = self.get_sided_piece(P(y, x));
                if !piece.is_first() {
                    continue;
                }
                let piece = piece.to_piece();
                let moves = self.enumerate_moves(P(y, x));

                for &mv in &moves {
                    let mut board = self.clone();
                    board.apply_move(mv);
                    if board.is_check() {
                        ret.push(mv);
                    }
                }
            }
        }
        let king_pos = self.locate_second_king();
        for i in 0..PIECE_TYPES {
            if self.hand_first[i] == 0 {
                continue;
            }
            for j in 0..PIECE_MOVES_COUNT {
                if PIECE_MOVES[i][j] == D(0, 0) {
                    break;
                }
                let pos = king_pos + PIECE_MOVES[i][j].flip();
                if !self.is_inside_board(pos) {
                    continue;
                }
                if self.get_sided_piece(pos).is_empty() {
                    ret.push(Move::FromHand(pos, SidedPiece(i as i8)));
                }
            }
        }
        if self.hand_first[PIECE_LANCE.0 as usize] != 0 {
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(1, 0),
                PIECE_LANCE.as_first(),
                &mut ret,
            );
        }
        if self.hand_first[PIECE_BISHOP.0 as usize] != 0 {
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(-1, -1),
                PIECE_BISHOP.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(-1, 1),
                PIECE_BISHOP.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(1, -1),
                PIECE_BISHOP.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(1, 1),
                PIECE_BISHOP.as_first(),
                &mut ret,
            );
        }
        if self.hand_first[PIECE_ROOK.0 as usize] != 0 {
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(-1, 0),
                PIECE_ROOK.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(0, -1),
                PIECE_ROOK.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(0, 1),
                PIECE_ROOK.as_first(),
                &mut ret,
            );
            self.enumerate_check_from_hand_ranging(
                king_pos,
                D(1, 0),
                PIECE_ROOK.as_first(),
                &mut ret,
            );
        }
        ret
    }
    pub fn enumerate_check_from_hand_ranging(
        &self,
        pos: P,
        dir: D,
        piece: SidedPiece,
        dest: &mut Vec<Move>,
    ) {
        for i in 1..BOARD_SIZE {
            let pos2 = pos + dir * i;
            if !self.is_inside_board(pos2) {
                break;
            }
            if !self.get_sided_piece(pos2).is_empty() {
                break;
            }
            dest.push(Move::FromHand(pos2, piece));
        }
    }
    pub fn enumerate_check_avoidance(&self) -> Vec<Move> {
        // TODO: more efficient algorithm
        let mut ret = vec![];
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let piece = self.get_sided_piece(P(y, x));
                if !piece.is_second() {
                    continue;
                }
                let piece = piece.to_piece();
                let moves = self.enumerate_moves(P(y, x));

                for &mv in &moves {
                    let mut board = self.clone();
                    board.apply_move(mv);
                    if !board.is_check() {
                        ret.push(mv);
                    }
                }
            }
        }
        ret
    }
    pub fn is_checkmate(&self) -> bool {
        self.enumerate_check_avoidance().len() == 0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let piece = self.get_sided_piece(P(y, x));
                if piece.is_empty() {
                    write!(f, " .. ")?;
                } else if piece.is_first() {
                    write!(f, " {} ", piece.to_piece().get_name())?;
                } else if piece.is_second() {
                    write!(f, "v{} ", piece.to_piece().get_name())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_check() {
        let mut board = Board::new();
        board.set_sided_piece(P(0, 4), PIECE_KING.as_second());
        board.set_sided_piece(P(2, 5), PIECE_GOLD.as_first());

        let checks = board.enumerate_check();
        assert_eq!(checks.len(), 2);
        assert!(checks.contains(&Move::NoPromotion(P(2, 5), P(1, 4))));
        assert!(checks.contains(&Move::NoPromotion(P(2, 5), P(1, 5))));
    }

    #[test]
    fn test_enumerate_check_ranging() {
        {
            // lance and promotion
            let mut board = Board::new();
            board.set_sided_piece(P(1, 1), PIECE_KING.as_second());
            board.set_sided_piece(P(8, 0), PIECE_LANCE.as_first());

            let checks = board.enumerate_check();
            assert_eq!(checks.len(), 2);
            assert!(checks.contains(&Move::Promotion(P(8, 0), P(2, 0))));
            assert!(checks.contains(&Move::Promotion(P(8, 0), P(1, 0))));
        }
        {
            // bishop
            let mut board = Board::new();
            board.set_sided_piece(P(2, 2), PIECE_KING.as_second());
            board.set_sided_piece(P(8, 2), PIECE_BISHOP.as_first());

            let checks = board.enumerate_check();
            assert_eq!(checks.len(), 1);
            assert!(checks.contains(&Move::NoPromotion(P(8, 2), P(5, 5))));
        }
    }
}
