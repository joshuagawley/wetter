pub const MIN_WIDTH: usize = 34;
pub const MIN_CELL_WIDTH: usize = MIN_WIDTH / 2 - 2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Dimensions {
    pub term_width: usize,
    pub cell_width: usize,
}

impl From<Dimensions> for (usize, usize) {
    fn from(value: Dimensions) -> Self {
        (value.term_width, value.cell_width)
    }
}

impl Dimensions {
    pub(crate) fn new(term_width: usize, cell_width: usize) -> Self {
        Self {
            term_width,
            cell_width,
        }
    }
}
