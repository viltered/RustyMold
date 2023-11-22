use std::cell::RefCell;
use std::rc::Rc;

use arrayvec::ArrayVec;
use fastrand;

/// number of genes in each genome
const GENOME_SIZE: usize = 100;
/// increase in energy loss per tick for a cell per passing age
const ENERGY_LOSS: i32 = 5;
/// number of ticks elapsed before aging
const TICKS_TO_AGE: i32 = 200;
/// minimum age for spore to bloom
const SPORE_RIPING_AGE: u32 = 100;
/// chance that a gene will stop growth in a direction
const STOP_CHANCE: f32 = 0.5;
/// chance that a non-stopping gene will create a spore
const SPORE_CHANCE: f32 = 0.01;
/// chance of a mutation ocuring when a spore sprouts
const MUTATION_CHANCE: f32 = 1. / 50.;

const BACKGROUND_COLOR: u32 = 0;

#[derive(Clone)]
struct Genome {
    /// Genes of a mold. A gene is three numbers, one for each relative growth direction.
    /// Growth of a cell depends on the current active gene's values.
    /// -2: no growth.
    /// -1: create spore.
    /// 0 to GENOME_SIZE: growth with new active gene set to this value.
    genes: [isize; GENOME_SIZE * 3],
    /// A u32 representing the mold's color using the pattern 0RGB: one byte of zeros, and one byte for red, green and blue.
    color: u32,
}

struct Mold {
    genome: Rc<Genome>,
    energy: RefCell<i32>,
}

#[derive(Clone)]
enum Cell {
    Empty,
    Spore {
        mold: Rc<Mold>,
        age: u32,
        direction: u32,
    },
    MoldPart {
        mold: Rc<Mold>,
        age: u32,
        active_gene: u32,
        direction: u32,
    },
}

/// Randomly generate a single gene
fn generate_gene() -> isize {
    if fastrand::f32() < STOP_CHANCE {
        -2
    } else if fastrand::f32() < SPORE_CHANCE {
        -1
    } else {
        fastrand::isize(0..GENOME_SIZE as isize)
    }
}

impl Genome {
    /// Create a new genome by mutating this one.
    fn make_mutation(&self) -> Genome {
        let mut new_genome = self.clone();
        if fastrand::f32() < MUTATION_CHANCE {
            new_genome.color = (10 + fastrand::u32(0..236) << 16)
                | (10 + fastrand::u32(0..236) << 8)
                | (10 + fastrand::u32(0..236));
            let mutation_location = fastrand::usize(0..(GENOME_SIZE * 3));
            new_genome.genes[mutation_location] = generate_gene();
        }
        return new_genome;
    }

    /// Randomly generate a new genome.
    fn new() -> Self {
        let mut genome = Self {
            genes: [0; GENOME_SIZE * 3],

            color: (10 + fastrand::u32(0..236) << 16)
                | (10 + fastrand::u32(0..236) << 8)
                | (10 + fastrand::u32(0..236)),
        };
        for gene in genome.genes.iter_mut() {
            *gene = generate_gene();
        }
        genome
    }
}

/// Full simulation state.
pub struct Simulation {
    energy_light: i32,
    grid: Vec<Vec<Cell>>,
    size_x: usize,
    size_y: usize,
}

impl Simulation {
    pub fn new(size_x: usize, size_y: usize, energy_light: i32) -> Self {
        let mut s = Simulation {
            energy_light: energy_light,
            grid: Vec::new(),
            size_x: size_x,
            size_y: size_y,
        };
        for _ in 0..size_x {
            let mut v = Vec::new();
            for _ in 0..size_y {
                v.push(Cell::Empty);
            }
            s.grid.push(v);
        }
        s
    }

    /// If position (x, y) is empty, create a new mold with a newly generated genome and return true.
    /// If (x, y) is occupied, return false.
    pub fn generate_mold(&mut self, x: usize, y: usize) -> bool {
        match self.grid[x][y] {
            Cell::Empty => {
                let genome = Genome::new();
                let mold = Mold {
                    genome: Rc::new(genome),
                    energy: RefCell::new(0),
                };
                let cell = Cell::MoldPart {
                    mold: Rc::new(mold),
                    age: 0,
                    active_gene: 0,
                    direction: 0,
                };
                self.grid[x][y] = cell;
                true
            }
            _ => false,
        }
    }

    pub fn clear(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                self.grid[x][y] = Cell::Empty;
            }
        }
    }

    /// Evolve the state of the simulation forward by one time step.
    pub fn update(&mut self) {
        // first pass: increase age, apply energy cost, give energy from empty cells
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                match self.grid[x][y] {
                    Cell::MoldPart {
                        ref mut age,
                        ref mold,
                        ..
                    }
                    | Cell::Spore {
                        ref mut age,
                        ref mold,
                        ..
                    } => {
                        *mold.energy.borrow_mut() -= ENERGY_LOSS * (1 + *age as i32 / TICKS_TO_AGE);
                        *age += 1;
                    }
                    Cell::Empty => {
                        self.distribute_energy(x, y);
                    }
                }
            }
        }

        // second pass: grow molds, remove molds that are out of energy and awaken their spores
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                match &self.grid[x][y].clone() {
                    Cell::Spore {
                        mold,
                        age,
                        direction,
                    } if *mold.energy.borrow() <= 0 => {
                        if *age >= SPORE_RIPING_AGE {
                            self.grid[x][y] = Cell::MoldPart {
                                mold: Rc::new(Mold {
                                    genome: Rc::new((*mold.genome).make_mutation()),
                                    energy: RefCell::new(0),
                                }),
                                age: 0,
                                active_gene: 0,
                                direction: *direction,
                            }
                        } else {
                            self.grid[x][y] = Cell::Empty;
                        }
                    }
                    Cell::MoldPart { mold, .. } if *mold.energy.borrow() <= 0 => {
                        self.grid[x][y] = Cell::Empty;
                    }
                    Cell::MoldPart {
                        mold,
                        age,
                        active_gene,
                        direction,
                    } if *age > 0 => {
                        // todo: make void grow from neighboring cells to make grid[x][y] the only modified cell
                        for rel_grow_direction in 0..3 {
                            let next_active_gene =
                                mold.genome.genes[*active_gene as usize * 3 + rel_grow_direction];

                            // gene -2 indicates no growth in this direction
                            if next_active_gene < -1 {
                                continue;
                            }

                            // target_offset (with size of canvas added to ensure positive values)
                            let abs_grow_direction =
                                (3 + *direction + rel_grow_direction as u32) % 4;
                            let (target_dx, target_dy): (usize, usize) = match abs_grow_direction {
                                0 => (self.size_x, self.size_y + 1),
                                1 => (self.size_x + 1, self.size_y),
                                2 => (self.size_x, self.size_y - 1),
                                3.. => (self.size_x - 1, self.size_y),
                            };
                            let target_x = (x + target_dx) % self.size_x;
                            let target_y = (y + target_dy) % self.size_y;

                            // if target cell is empty, add new MoldPart or spore referring to the same mold
                            if matches!(&self.grid[target_x][target_y], Cell::Empty) {
                                if next_active_gene == -1 {
                                    self.grid[target_x][target_y] = Cell::Spore {
                                        mold: mold.clone(),
                                        age: 0,
                                        direction: abs_grow_direction,
                                    };
                                } else {
                                    self.grid[target_x][target_y] = Cell::MoldPart {
                                        mold: mold.clone(),
                                        age: 0,
                                        active_gene: next_active_gene as u32,
                                        direction: abs_grow_direction,
                                    };
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    /// If there is only one mold neighboring (x, y), give it energy equal to energy_light.
    #[inline]
    fn distribute_energy(&mut self, x: usize, y: usize) {
        let mut neighbors: ArrayVec<Rc<Mold>, 4> = ArrayVec::new();

        let offsets: [(usize, usize); 4] = [
            (self.size_x, self.size_y + 1),
            (self.size_x + 1, self.size_y),
            (self.size_x, self.size_y - 1),
            (self.size_x - 1, self.size_y),
        ];
        for (dx, dy) in offsets.iter() {
            let n = &self.grid[(x + dx) % self.size_x][(y + dy) % self.size_y];
            if let Cell::MoldPart { mold, .. } | Cell::Spore { mold, .. } = n {
                if neighbors
                    .iter()
                    .all(|neighbor| !Rc::ptr_eq(&neighbor, mold))
                {
                    neighbors.push(mold.clone());
                }
            }
        }
        if neighbors.len() == 1 {
            *neighbors[0].energy.borrow_mut() += self.energy_light;
        }
    }

    /// Render the state of the simulation into a buffer.
    pub fn render(&self, buffer: &mut Vec<u32>, window_x: usize, window_y: usize) {
        let mut buffer_index = 0;
        for y in 0..window_y {
            for x in 0..window_x {
                // todo: pan/zoom
                let x_grid = std::cmp::min(x, window_x);
                let y_grid = std::cmp::min(y, window_y);

                match &self.grid[x_grid][y_grid] {
                    Cell::Empty => {
                        buffer[buffer_index] = BACKGROUND_COLOR;
                    }
                    Cell::Spore { mold, age, .. } if *age >= SPORE_RIPING_AGE => {
                        // invert color with boolean NOT to distinguish spores from normal cells
                        buffer[buffer_index] = !mold.genome.color;
                    }
                    Cell::MoldPart { mold, .. } | Cell::Spore { mold, .. } => {
                        buffer[buffer_index] = mold.genome.color;
                    }
                }

                buffer_index += 1;
            }
        }
    }
}
