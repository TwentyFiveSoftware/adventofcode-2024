use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let disk_map = DiskMap::from_str(input).unwrap();

    println!("PART 1: {}", part1(disk_map.clone()));
    println!("PART 2: {}", part2(disk_map));
}

fn part1(mut disk_map: DiskMap) -> usize {
    loop {
        let mut region = disk_map.regions.pop().unwrap();
        if region.id.is_none() {
            continue;
        }

        let mut reordering_successful = false;

        while let Some(free_region_index) = disk_map.find_first_free_region_index(1) {
            disk_map.regions[free_region_index].id = region.id;

            if disk_map.regions[free_region_index].size() < region.size() {
                region.end -= disk_map.regions[free_region_index].size();
                continue;
            }

            if disk_map.regions[free_region_index].size() > region.size() {
                disk_map.split_free_region(free_region_index, region.size());
            }

            reordering_successful = true;
            break;
        }

        if !reordering_successful {
            disk_map.regions.push(region);
            break;
        }
    }

    disk_map.checksum()
}

fn part2(mut disk_map: DiskMap) -> usize {
    disk_map.regions.clone().iter()
        .rev()
        .filter(|region| region.id.is_some())
        .for_each(|region| {
            let Some(free_region_index) = disk_map.find_first_free_region_index(region.size()) else { return; };

            if disk_map.regions[free_region_index].start > region.start {
                return;
            }

            disk_map.regions[free_region_index].id = region.id;

            if disk_map.regions[free_region_index].size() > region.size() {
                disk_map.split_free_region(free_region_index, region.size());
            }

            disk_map.free_region(region);
        });

    disk_map.checksum()
}

#[derive(Clone)]
struct DiskMap {
    regions: Vec<DiskRegion>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.regions.iter().map(|region| region.checksum()).sum()
    }

    fn find_first_free_region_index(&self, min_size: usize) -> Option<usize> {
        self.regions.iter().position(|region| region.id.is_none() && region.size() >= min_size)
    }

    fn free_region(&mut self, region: &DiskRegion) {
        let Some(index) = self.regions.iter().rposition(|other_region| other_region.id == region.id) else { return; };
        self.regions[index].id = None;
    }

    fn split_free_region(&mut self, free_region_index: usize, first_half_size: usize) {
        let new_end = self.regions[free_region_index].start + first_half_size - 1;

        self.regions.insert(free_region_index + 1, DiskRegion {
            id: None,
            start: new_end + 1,
            end: self.regions[free_region_index].end,
        });

        self.regions[free_region_index].end = new_end;
    }
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut regions: Vec<DiskRegion> = vec![];

        let mut next_block_start_index: usize = 0;
        let mut next_file_id: usize = 0;

        s.char_indices().for_each(|(index, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if size == 0 {
                return;
            }

            let id = if index % 2 == 1 {
                None
            } else {
                let file_id = next_file_id;
                next_file_id += 1;

                Some(file_id)
            };

            regions.push(DiskRegion {
                id,
                start: next_block_start_index,
                end: next_block_start_index + size - 1,
            });

            next_block_start_index += size;
        });

        Ok(DiskMap { regions })
    }
}

#[derive(Clone)]
struct DiskRegion {
    id: Option<usize>,
    start: usize,
    end: usize,
}

impl DiskRegion {
    fn size(&self) -> usize {
        self.end - self.start + 1
    }

    fn checksum(&self) -> usize {
        match self.id {
            None => 0,
            Some(id) => (self.start..(self.end + 1)).sum::<usize>() * id
        }
    }
}
