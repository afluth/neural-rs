use std::fmt;
use std::ops::{Index, IndexMut};
use rand::{Rand, Rng};

/// A Tic-Tac-Toe mark
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub enum Mark {
    X,
    O,
    None,
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mark::X => write!(f, "X"),
            Mark::O => write!(f, "O"),
            Mark::None => write!(f, " "),
        }
    }
}

/// A standard Tic-Tac-Toe game board which is indexed like so:
///
/// 0 | 1 | 2
/// --+---+--
/// 3 | 4 | 5
/// --+---+--
/// 6 | 7 | 8
pub struct Board([Mark; 9]);

impl Board {
    
    /// Constructs a new `Board` with every space populated with `None`.
    pub fn new() -> Board {
        Board([Mark::None; 9])
    }

	/// Provides a view of this `Board` from the given `Rotation`
    pub fn get_view(&mut self, rotation: Rotation) -> View {
        View {
            rotation: rotation,
            board: self,
        }
    }
}

/// A view of the game board
pub struct View<'a> {
    rotation: Rotation,
    board: &'a mut Board,
}

impl<'a> Index<usize> for View<'a> {
    type Output = Mark;

    fn index(&self, i: usize) -> &Mark {
        let &mut Board(ref board_array) = self.board;

        &board_array[self.rotation.apply(i)]
    }
}

impl<'a> IndexMut<usize> for View<'a> {
    fn index_mut(&mut self, i: usize) -> &mut Mark {
        let &mut Board(ref mut board_array) = self.board;

        &mut board_array[self.rotation.apply(i)]
    }
}

const TOP: [usize; 9] = [8, 7, 6, 5, 4, 3, 2, 1, 0];
const BOTTOM: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const LEFT: [usize; 9] = [2, 5, 8, 1, 4, 7, 0, 3, 6];
const RIGHT: [usize; 9] = [6, 3, 0, 7, 4, 1, 8, 5, 2];

#[derive(Copy, Clone)]
pub enum Rotation {
    Top,
    Bottom,
    Left,
    Right,
}

impl Rotation {
    fn apply(&self, index: usize) -> usize {
        match *self {
            Rotation::Top => TOP[index],
            Rotation::Bottom => BOTTOM[index],
            Rotation::Left => LEFT[index],
            Rotation::Right => RIGHT[index],
        }
    }
}

impl Rand for Rotation {
    fn rand<R: Rng>(rng: &mut R) -> Self {

        match rng.gen_range(0, 4) {
            0 => Rotation::Bottom,
            1 => Rotation::Left,
            2 => Rotation::Top,
            3 => Rotation::Right,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn apply_rotation_top() {
        let top = Rotation::Top;
        
        assert_eq!(6, top.apply(2));
        assert_eq!(4, top.apply(4));
        assert_eq!(2, top.apply(6));
    }
    
    #[test]
    fn apply_rotation_left() {
        let left = Rotation::Left;
        
        assert_eq!(8, left.apply(2));
        assert_eq!(4, left.apply(4));
        assert_eq!(0, left.apply(6));
    }
        
    #[test]
    fn apply_rotation_right() {
        let right = Rotation::Right;
        
        assert_eq!(0, right.apply(2));
        assert_eq!(4, right.apply(4));
        assert_eq!(8, right.apply(6));
    }
    
    #[test]
    fn view_top() {
        let mut board = Board::new();
        board.0[2] = Mark::X;
        
        {
        	let mut view = board.get_view(Rotation::Top);
        	view[2] = Mark::O;
        	assert_eq!(Mark::X, view[6]);
        }
        
        assert_eq!(Mark::O, board.0[6]);
    }
}
