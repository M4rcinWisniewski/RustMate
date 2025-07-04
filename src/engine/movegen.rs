use crate::board::{
    PieceType,
    Color,
    Bitboards
};



const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = 0x0202020202020202;
const FILE_G: u64 = 0x4040404040404040;
const FILE_H: u64 = 0x8080808080808080;
#[derive(Debug)]
pub struct Move {
    from: u8,
    to: u8,
    piece: PieceType,
    promotion_rights: bool
}



impl Move {


    pub fn generate_moves_for_piece(sq: u8, piece: PieceType, color: Color, boards: &Bitboards) -> Vec<Move> {
        match piece {
            PieceType::Pawn => Self::pawn_moves(sq, color, boards),
            PieceType::Knight => Self::knight_moves(sq, color, boards),
            PieceType::King => Self::king_moves(sq, color, boards),
            PieceType::Rook => Self::rook_moves(sq, color, boards),
            PieceType::Bishop => Self::bishop_moves(sq, color, boards),
            PieceType::Queen => Self::queen_moves(sq, color, boards),
            _ => Vec::new(), // default empty for unimplemented
        }
    }

    fn is_square_occupied(bitboard: u64, square: u8) -> bool {
    (bitboard & (1u64 << square)) != 0
    }

    fn get_own_pieces(bitboards: &Bitboards, color: Color) -> u64 {
        bitboards.boards[color as usize]
            .iter()
            .fold(0u64, |acc, &bb| acc | bb)
    }  

    fn get_opponent_pieces(bitboards: &Bitboards, color: Color) -> u64 {
        let opponent = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        bitboards.boards[opponent as usize]
            .iter()
            .fold(0u64, |acc, &bb| acc | bb)
    }


    // return board representation of squares that a passed knight can move to
    fn knight_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> { 
        let knight= 1u64 << sq;
        let mut moves = 0u64;
        let own_pieces = Self::get_own_pieces(board, color);

        if !Self::is_square_occupied(own_pieces, sq + 17) {
            moves |= (knight & !(FILE_H)) << 17; // ↑2 →1
        } 
        if !Self::is_square_occupied(own_pieces, sq + 15) {
            moves |= (knight & !(FILE_A))             << 15; // ↑2 ←1
        } 
        if !Self::is_square_occupied(own_pieces, sq + 10) {
            moves |= (knight & !(FILE_G | FILE_H))    << 10; // ↑1 →2
        } 
        if !Self::is_square_occupied(own_pieces, sq + 6) {
            moves |= (knight & !(FILE_A | FILE_B))    << 6;  // ↑1 ←2
        }
        if !Self::is_square_occupied(own_pieces, sq - 17) {
            moves |= (knight & !(FILE_A)) >> 17; // ↓2 ←1;
        } 
        if !Self::is_square_occupied(own_pieces, sq - 15) {
            moves |= (knight & !(FILE_H))             >> 15; // ↓2 →1
        }
        if !Self::is_square_occupied(own_pieces, sq - 10) {
            moves |= (knight & !(FILE_A | FILE_B))    >> 10; // ↓1 ←2
        } 
        if !Self::is_square_occupied(own_pieces, sq - 6) {
            moves |= (knight & !(FILE_G | FILE_H))    >> 6;  // ↓1 →2
        }
        
        Self::moves_from_bitboard(sq, PieceType::Knight, moves, false)
    }


    fn king_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> {
        let king = 1u64 << sq;
        let mut moves = 0u64;
        let own_pieces = Self::get_own_pieces(board, color);

        // Horizontal moves
        if !Self::is_square_occupied(own_pieces, sq + 1) {
            moves |= (king & !FILE_H) << 1;  // East
        }
        if !Self::is_square_occupied(own_pieces, sq - 1) {
            moves |= (king & !FILE_A) >> 1;  // West
        }
        // Vertical moves
        if !Self::is_square_occupied(own_pieces, sq + 8) {
            moves |= king << 8; // North
        }  
        if !Self::is_square_occupied(own_pieces, sq - 8) {     
            moves |= king >> 8;             // South
        }
        // Diagonal moves
        if !Self::is_square_occupied(own_pieces, sq + 9) {
            moves |= (king & !FILE_H) << 9;  // North-East
        }
        if !Self::is_square_occupied(own_pieces, sq + 7) {
            moves |= (king & !FILE_A) << 7;  // North-West
        }
        if !Self::is_square_occupied(own_pieces, sq - 8) {
            moves |= (king & !FILE_H) >> 7;  // South-East
        }
        if !Self::is_square_occupied(own_pieces, sq + 8) {
            moves |= (king & !FILE_A) >> 9;  // South-West
        }

        Self::moves_from_bitboard(sq, PieceType::King, moves, false)
    }


    fn pawn_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> {
        //Make a bitboard representing all the pieces in the board
        let all_pieces: u64 = Self::get_own_pieces(board, color);
        let opponet_pieces = Self::get_opponent_pieces(board, color);

        let pawn = 1u64 << sq;
        let mut moves = 0u64;
        let promotion: bool;


        if color == Color::White {
            if !Self::is_square_occupied(all_pieces, sq + 8) { //check if the square above a pawn is not occupied
                moves |= pawn << 8; // one-square move
                if !Self::is_square_occupied(all_pieces, sq + 16) {
                    moves |= pawn << 16; // two-square move
                }
            }
            if Self::is_square_occupied(opponet_pieces, sq + 9) {
                moves |= (pawn & !FILE_H) << 9;
            }
            if Self::is_square_occupied(opponet_pieces, sq + 7) {
                moves |= (pawn & !FILE_A) << 7;
            }

        } else {
            if !Self::is_square_occupied(all_pieces, sq - 8) { //check if the square above a pawn is not occupied
                moves |= pawn >> 8; // one-square move
                if !Self::is_square_occupied(all_pieces, sq - 16) {
                    moves |= pawn >> 16; // two-square move
                }
            }
            if Self::is_square_occupied(opponet_pieces, sq - 9) {
                moves |= (pawn & !FILE_H) >> 9; // Takes on diagonal to the east
            }
            if Self::is_square_occupied(opponet_pieces, sq - 7) {
                moves |= (pawn & !FILE_A) >> 7; //Takes on diagonal to the west
            }

        }
        if  (color == Color::White && sq >= 48 && sq <= 55) || 
            (color == Color::Black && sq >= 8 && sq <= 15) {
                promotion = true;
            } else {
                promotion = false;
            }


        Self::moves_from_bitboard(sq, PieceType::Pawn, moves, promotion)
    }

    pub fn rook_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> {
        let mut moves = 0u64;
        let own_pieces = Self::get_own_pieces(board, color);
        let enemy_pieces = Self::get_opponent_pieces(board, color);

        // NORTH
        let mut next_sq = sq + 8;
        while next_sq <= 63 {
            if Self::is_square_occupied(own_pieces, next_sq) {
                break;
            }
            moves |= 1u64 << next_sq;
            if Self::is_square_occupied(enemy_pieces, next_sq) {
                break;
            }
            next_sq += 8;
        }

        // SOUTH
        let mut next_sq = sq.wrapping_sub(8);
        while next_sq <= 63 {
            if Self::is_square_occupied(own_pieces, next_sq) {
                break;
            }
            moves |= 1u64 << next_sq;
            if Self::is_square_occupied(enemy_pieces, next_sq) {
                break;
            }
            if next_sq < 8 { break; }
            next_sq -= 8;
        }

        // EAST
        let mut next_sq = sq + 1;
        while next_sq % 8 != 0 {
            if Self::is_square_occupied(own_pieces, next_sq) {
                break;
            }
            moves |= 1u64 << next_sq;
            if Self::is_square_occupied(enemy_pieces, next_sq) {
                break;
            }
            next_sq += 1;
        }

        // WEST
        let mut next_sq = sq.wrapping_sub(1);
        while sq % 8 != 0 && next_sq % 8 != 7 {
            if Self::is_square_occupied(own_pieces, next_sq) {
                break;
            }
            moves |= 1u64 << next_sq;
            if Self::is_square_occupied(enemy_pieces, next_sq) {
                break;
            }
            if next_sq == 0 { break; }
            next_sq -= 1;
        }

        Self::moves_from_bitboard(sq, PieceType::Rook, moves, false)
    }



    pub fn bishop_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> {
        let mut moves = 0u64;

        let own_pieces = Self::get_own_pieces(board, color);
        let enemy_pieces = Self::get_opponent_pieces(board, color);

        // Northeast (+9)
        let mut pos = sq;
        while pos < 56 && (1u64 << pos) & FILE_H == 0 {
            pos += 9;
            if pos >= 64 {
                break;
            }
            if Self::is_square_occupied(own_pieces, pos) {
                break;
            }
            moves |= 1u64 << pos;
            if Self::is_square_occupied(enemy_pieces, pos) {
                break;
            }
            if pos % 8 == 7 { break; } // prevent edge wrapping
        }

        // Northwest (+7)
        pos = sq;
        while pos < 56 && (1u64 << pos) & FILE_A == 0 {
            pos += 7;
            if pos >= 64 {
                break;
            }
            if Self::is_square_occupied(own_pieces, pos) {
                break;
            }
            moves |= 1u64 << pos;
            if Self::is_square_occupied(enemy_pieces, pos) {
                break;
            }
            if pos % 8 == 0 { break; } // prevent edge wrapping
        }

        // Southeast (-7)
        pos = sq;
        while pos >= 7 && (1u64 << pos) & FILE_H == 0 {
            pos = pos.wrapping_sub(7);
            if Self::is_square_occupied(own_pieces, pos) {
                break;
            }
            moves |= 1u64 << pos;
            if Self::is_square_occupied(enemy_pieces, pos) {
                break;
            }
            if pos % 8 == 7 { break; }
            if pos < 7 { break; }
        }

        // Southwest (-9)
        pos = sq;
        while pos >= 9 && (1u64 << pos) & FILE_A == 0 {
            pos = pos.wrapping_sub(9);
            if Self::is_square_occupied(own_pieces, pos) {
                break;
            }
            moves |= 1u64 << pos;
            if Self::is_square_occupied(enemy_pieces, pos) {
                break;
            }
            if pos % 8 == 0 { break; }
            if pos < 9 { break; }
        }

        Self::moves_from_bitboard(sq, PieceType::Bishop, moves, false)
    }



    fn queen_moves(sq: u8, color: Color, board: &Bitboards) -> Vec<Move> {
        let mut moves = Move::rook_moves(sq, color, board);
        moves.extend(Move::bishop_moves(sq, color, board));
        moves
    }


    fn moves_from_bitboard(
        from_sq: u8,
        piece: PieceType,
        destinations: u64,
        promotion_rights: bool,

        ) -> Vec<Move> {
        
        let mut moves_vec = Vec::new();
        let mut bitboard_copy = destinations;

        while bitboard_copy != 0 {
            let to_square = bitboard_copy.trailing_zeros() as u8;
            bitboard_copy &= bitboard_copy - 1;

            let m = Move {
                from: from_sq,
                to: to_square,
                piece,
                promotion_rights: promotion_rights,

            };

            moves_vec.push(m);
        }

        moves_vec
    }

}

