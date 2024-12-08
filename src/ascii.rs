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
    pub fn right45(&self) -> Dir {
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

    pub fn right90(&self) -> Dir {
        self.right45().right45()
    }

    pub fn rev(&self) -> Dir {
        self.right90().right90()
    }

    pub fn left90(&self) -> Dir {
        self.rev().right90()
    }

    pub fn left45(&self) -> Dir {
        self.left90().right45()
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

    pub fn sub(&self, other: &Pos) -> (i64, i64) {
        return (self.0 - other.0, self.1 - other.1);
    }
}

pub struct Map {
    pub width: i64,
    pub height: i64,
    content: Vec<Vec<char>>,
    background: char,
}

#[allow(dead_code)]
impl Map {
    pub fn new(content: Vec<Vec<char>>) -> Map {
        assert!(!content.is_empty());
        assert!(!content.get(0).unwrap().is_empty());
        let height = content.len() as i64;
        let width = content.get(0).unwrap().len() as i64;
        assert!(content.iter().all(|x| x.len() as i64 == width));
        Map {
            width,
            height,
            content,
            background: ' ',
        }
    }

    pub fn valid_pos(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    pub fn get(&self, pos: Pos) -> char {
        if self.valid_pos(pos) {
            self.content[pos.1 as usize][pos.0 as usize].clone()
        } else {
            self.background
        }
    }

    pub fn empty(&self, pos: Pos) -> bool {
        self.get(pos) == self.background
    }

    pub fn set(&mut self, pos: Pos, c: char) {
        assert!(self.valid_pos(pos));
        self.content[pos.1 as usize][pos.0 as usize] = c
    }

    pub fn clear(&mut self, pos: Pos) {
        self.set(pos, self.background)
    }

    pub fn set_background(&mut self, bg: char) {
        self.background = bg;
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

    pub fn wrap(&self, p: Pos) -> Pos {
        p.wrap(self.width, self.height)
    }

    pub fn find(&self, c: char) -> Vec<Pos> {
        let mut res = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos(x, y);
                if self.get(pos) == c {
                    res.push(pos);
                }
            }
        }
        res
    }

    pub fn enumerate(&self) -> MapIterator {
        MapIterator {
            index: 0,
            map: self,
        }
    }
}

pub struct MapIterator<'a> {
    index: i64,
    map: &'a Map,
}

impl Iterator for MapIterator<'_> {
    type Item = (Pos, char);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.index % self.map.width;
        let y = self.index / self.map.width;
        self.index += 1;
        if y >= self.map.height {
            None
        } else {
            let p = Pos(x, y);
            Some((p, self.map.get(p)))
        }
    }
}
