use std::ops::Index;
use std::ops::IndexMut;
use rand::{Rand, Rng};

pub type Board = [char; 9];

pub struct View<'a> {
    rotation: Rotation,
    board: &'a mut Board,
}

pub fn get_view(board: &mut Board, rotation: Rotation) -> View {
    View {
        rotation: rotation,
        board: board,
    }
}

impl<'a> Index<usize> for View<'a> {
    type Output = char;

    fn index(&self, i: usize) -> &char {
        &self.board[self.rotation.apply(i)]
    }
}

impl<'a> IndexMut<usize> for View<'a> {
    fn index_mut(&mut self, i: usize) -> &mut char {
        &mut self.board[self.rotation.apply(i)]
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
