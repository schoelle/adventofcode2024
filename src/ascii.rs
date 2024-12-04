use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
#[allow(dead_code)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[allow(dead_code)]
impl Dir {
    pub fn left45(&self) -> Dir {
        match self {
            Dir::N => Dir::NE,
            Dir::NE => Dir::E,
            Dir::E => Dir::SE,
            Dir::SE => Dir::S,
            Dir::S => Dir::SW,
            Dir::SW => Dir::W,
            Dir::W => Dir::NW,
            Dir::NW => Dir::N,
        }
    }

    pub fn left90(&self) -> Dir {
        self.left45().left45()
    }

    pub fn rev(&self) -> Dir {
        self.left90().left90()
    }

    pub fn right90(&self) -> Dir {
        self.rev().left90()
    }

    pub fn right45(&self) -> Dir {
        self.right90().left45()
    }

    pub fn to_char(&self) -> char {
        match self {
            Dir::N => '^',
            Dir::NE => '?',
            Dir::E => '>',
            Dir::SE => '?',
            Dir::S => 'v',
            Dir::SW => '?',
            Dir::W => '<',
            Dir::NW => '?',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i64, pub i64);

#[allow(dead_code)]
impl Pos {
    pub fn wrap(&self, width: i64, height: i64) -> Pos {
        Pos(self.0.rem_euclid(width), self.0.rem_euclid(height))
    }

    pub fn nstep(&self, dir: Dir, cnt: i64) -> Pos {
        match dir {
            Dir::N => Pos(self.0, self.1 - cnt),
            Dir::NE => Pos(self.0 + cnt, self.1 - cnt),
            Dir::E => Pos(self.0 + cnt, self.1),
            Dir::SE => Pos(self.0 + cnt, self.1 + cnt),
            Dir::S => Pos(self.0, self.1 + cnt),
            Dir::SW => Pos(self.0 - cnt, self.1 + cnt),
            Dir::W => Pos(self.0 - cnt, self.1),
            Dir::NW => Pos(self.0 - cnt, self.1 - cnt),
        }
    }

    pub fn step(&self, dir: Dir) -> Pos {
        self.nstep(dir, 1)
    }
}

pub struct Map {
    pub width: i64,
    pub height: i64,
    content: Vec<Vec<char>>,
}

#[allow(dead_code)]
impl Map {
    pub fn new(content: Vec<Vec<char>>) -> Map {
        let height = content.len() as i64;
        let width = content.get(0).unwrap().len() as i64;
        Map {
            width,
            height,
            content,
        }
    }

    pub fn get(&self, pos: Pos) -> char {
        self.content
            .get(pos.1 as usize)
            .unwrap()
            .get(pos.0 as usize)
            .unwrap().clone()
    }

    pub fn to_string(&self) -> String {
        self.content
            .iter()
            .map(|v| v.into_iter().collect::<String>() + "\n")
            .into_iter()
            .collect()
    }

    pub fn expand(&mut self, c: char) {
        self.width = self.width + 2;
        self.height = self.height + 2;
        let mut content: Vec<Vec<char>> = Vec::new();
        content.push(std::iter::repeat(c).take(self.width as usize).collect());
        for v in self.content.iter() {
            let mut vv = v.clone();
            vv.push(c);
            vv.insert(0, c);
            content.push(vv);
        }
        content.push(std::iter::repeat(c).take(self.width as usize).collect());
        self.content = content
    }
}