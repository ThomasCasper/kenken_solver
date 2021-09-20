use std::collections::{HashSet,HashMap};

#[derive(Debug,Clone)]
pub struct BlackList {
    bl: HashMap<usize,HashSet<u32>>
}

impl BlackList {
    pub fn new() -> Self {
        BlackList {
            bl: HashMap::new()
        }

    }

    pub fn get(&self, pos: &usize) -> HashSet<u32> {
        if let Some(hs) = self.bl.get(pos) {
            hs.clone()
        } else {
            HashSet::<u32>::new()
        }

    }

    pub fn insert(&mut self, pos:&Vec<usize>, digits: &HashSet<u32>) {
        //dimension is col or row?

        let new_pos: Vec<usize>;

        let phs:HashSet<usize>=pos.clone().into_iter().collect();

        let col = pos[0] % 10;
        let row=pos[0]-col;

        //get position to update in BL
        if col == pos[1]%10 {
            //Dimension: column
            new_pos = (col..90).step_by(10)
                .filter(|p| !phs.contains(p)) //get rid of given positions
                .collect();
        } else {
            //Dimension: row
            new_pos = (row..row+9)
                .filter(|p| !phs.contains(p)) //get rid of given positions
                .collect();
        }
        for p in new_pos {
            let mut new_hs:HashSet<u32> =digits.clone();
            if let Some(ohs)=self.bl.get(&p) {
                //join old an new digits
                new_hs.extend(ohs)
            }
            let _= self.bl.insert(p, new_hs);
        }

    }
}