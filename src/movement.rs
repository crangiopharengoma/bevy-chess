use crate::board::Square;
use crate::pieces::{Piece, PieceColour, PieceType};

pub fn is_move_valid(piece: &Piece, new_position: Square, pieces: Vec<Piece>) -> bool {
    if new_position == piece.pos || new_position.is_occupied(&pieces) == Some(piece.colour) {
        return false;
    }

    // TODO must move out of check if possible

    match piece.piece_type {
        PieceType::King => is_valid_for_king(piece, new_position),
        PieceType::Queen => is_valid_for_queen(piece, new_position, &pieces),
        PieceType::Bishop => is_valid_for_bishop(piece, new_position, &pieces),
        PieceType::Knight => is_valid_for_knight(piece, new_position),
        PieceType::Rook => is_valid_for_rook(piece, new_position, &pieces),
        PieceType::Pawn => is_valid_for_pawn(piece, new_position, &pieces),
    }
}

fn is_valid_for_king(piece: &Piece, new_position: Square) -> bool {
    piece.pos.is_adjacent(&new_position)
    // TODO castling
    // TODO prevent moving into check
}

fn is_valid_for_queen(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces)
        && (new_position.is_same_diagonal(&piece.pos)
            || new_position.is_same_file(&piece.pos)
            || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_bishop(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces) && new_position.is_same_diagonal(&piece.pos)
}

fn is_valid_for_rook(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    is_path_empty(piece.pos, new_position, pieces)
        && (new_position.is_same_file(&piece.pos) || new_position.is_same_rank(&piece.pos))
}

fn is_valid_for_knight(piece: &Piece, new_position: Square) -> bool {
    ((piece.pos.x - new_position.x).abs() == 2 && (piece.pos.y - new_position.y).abs() == 1)
        || ((piece.pos.x - new_position.x).abs() == 1 && (piece.pos.y - new_position.y).abs() == 2)
}

fn is_valid_for_pawn(piece: &Piece, new_position: Square, pieces: &[Piece]) -> bool {
    let (movement_direction, start_x) = match piece.colour {
        PieceColour::White => (1, 1),
        PieceColour::Black => (-1, 6),
    };

    // Standard
    if new_position.x - piece.pos.x == movement_direction && piece.pos.y == new_position.y {
        return new_position.is_occupied(pieces).is_none();
    }

    // Starting
    if piece.pos.x == start_x
        && new_position.x - piece.pos.x == (2 * movement_direction)
        && piece.pos.y == new_position.y
        && is_path_empty(piece.pos, new_position, pieces)
    {
        return new_position.is_occupied(pieces).is_none();
    }

    // Taking
    if new_position.x - piece.pos.x == movement_direction
        && (piece.pos.y - new_position.y).abs() == 1
    {
        return new_position.is_occupied(pieces) == Some(piece.colour.opponent());
    }

    false
    // TODO en passant
}

fn is_path_empty(begin: Square, end: Square, pieces: &[Piece]) -> bool {
    if begin.x == end.x {
        // moving along a rank
        !pieces.iter().any(|piece| {
            piece.pos.x == begin.x
                && ((piece.pos.y > begin.y && piece.pos.y < end.y)
                    || (piece.pos.y > end.y && piece.pos.y < begin.y))
        })
    } else if begin.y == end.y {
        // moving along a file
        !pieces.iter().any(|piece| {
            piece.pos.y == begin.y
                && ((piece.pos.x > begin.x && piece.pos.x < end.x)
                    || (piece.pos.x > end.x && piece.pos.x < begin.x))
        })
    } else {
        // diagonal
        let (x_diff, y_diff) = ((begin.x - end.x).abs(), (begin.y - end.y).abs());
        if x_diff == y_diff {
            for i in 1..x_diff {
                let pos: Square = if begin.x < end.x && begin.y < end.y {
                    // left bottom - right top
                    (begin.x + i, begin.y + i).into()
                } else if begin.x < end.x && begin.y > end.y {
                    // left top - right bottom
                    (begin.x + i, begin.y - i).into()
                } else if begin.x > end.x && begin.y < end.y {
                    // right bottom - left top
                    (begin.x - i, begin.y + i).into()
                } else {
                    // right top to left bottom
                    (begin.x - i, begin.y - i).into()
                };

                if pos.is_occupied(pieces).is_some() {
                    return false;
                }
            }
        }

        true
    }
}
