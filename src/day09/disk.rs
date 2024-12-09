use std::ops::{Add, AddAssign, SubAssign};

#[derive(Debug)]
pub struct Disk {
    size: usize,
    pub(crate) blocks: Vec<i32>,
    max_id: i32,
}

impl Disk {
    pub fn from_string(input: &str) -> Disk {
        let mut data = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .collect::<Vec<u32>>();

        let size = data.iter().map(|x| *x as usize).sum();

        let mut blocks: Vec<i32> = vec![];

        let mut id = 0;
        let mut free_block = false;

        while let Some(block) = data.pop() {
            if free_block {
                blocks.extend(vec![-1; block as usize])
            } else {
                blocks.extend(vec![id; block as usize]);
                id += 1;
            }
            free_block = !free_block;
        }

        Self {
            size,
            blocks,
            max_id: id - 1,
        }
    }

    pub fn checksum(&mut self) -> i64 {
        let mut checksum: i64 = 0;

        for i in 0..self.size {
            let block = self.blocks[i];
            if block == -1 {
                continue;
            }

            checksum += block as i64 * i as i64;
        }

        checksum
    }

    pub fn defragment(&mut self) {
        let mut left_ptr: usize = 0;
        let mut right_ptr: usize = self.size - 1;

        loop {
            while left_ptr < right_ptr && self.blocks[left_ptr] != -1 {
                left_ptr.add_assign(1);
            }

            while right_ptr > left_ptr && self.blocks[right_ptr] == -1 {
                right_ptr.sub_assign(1);
            }

            if left_ptr >= right_ptr {
                break;
            }

            self.blocks.swap(left_ptr, right_ptr);
        }
    }

    pub fn defragment_full(&mut self) {
        let free_ptr: usize = 0;
        let mut file_ptr: usize = self.size - 1;
        let mut last_id = self.max_id;

        while last_id >= 0 {
            let (file_block_position, file_block_size) =
                self.find_reverse_file_block_position_and_size_by_id(file_ptr, last_id);

            if let Some((free_block_position, _)) = self.find_free_block_position_and_size(
                free_ptr,
                file_block_position,
                file_block_size,
            ) {
                for i in 0..file_block_size as usize {
                    self.blocks
                        .swap(file_block_position.add(i), free_block_position.add(i));
                }
            }

            file_ptr = file_block_position;
            last_id.sub_assign(1);
        }
    }

    fn find_reverse_file_block_position_and_size_by_id(
        &self,
        from: usize,
        id: i32,
    ) -> (usize, i32) {
        if id == 0 {
            return (0, 0);
        }

        let mut pos: usize = from;
        while self.blocks[pos] != id {
            pos.sub_assign(1);
        }

        let mut size = 0;

        while pos > 0 && self.blocks[pos] == id {
            size.add_assign(1);
            pos.sub_assign(1);
        }

        (pos + 1, size)
    }

    fn find_first_free_block(&self, from: usize) -> Option<usize> {
        let mut pos: usize = from;
        while pos < self.size && self.blocks[pos] != -1 {
            pos.add_assign(1)
        }

        if pos < self.size {
            Some(pos)
        } else {
            None
        }
    }

    fn find_free_block_position_and_size(
        &self,
        from: usize,
        to: usize,
        required_size: i32,
    ) -> Option<(usize, i32)> {
        let mut orig = from;

        while let Some(mut pos) = self.find_first_free_block(orig) {
            let from = pos;
            let mut size = 0;

            while pos < to && self.blocks[pos] == -1 {
                size.add_assign(1);
                pos.add_assign(1);
            }

            if size >= required_size {
                return Some((from, size));
            }

            if pos >= to {
                break;
            }

            orig = pos;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_disk() {
        // Arrange
        let disk = Disk::from_string("2333133121414131402");
        // Act
        // Assert
        assert_eq!(disk.size, 42);
        assert_eq!(disk.blocks[0], 0);
        assert_eq!(disk.blocks[1], 0);
        assert_eq!(disk.blocks[2], -1);
        assert_eq!(disk.blocks[40], 9);
        assert_eq!(disk.blocks[41], 9);
    }

    #[test]
    fn test_defragment_full_disk() {
        // Arrange
        let mut disk = Disk::from_string("2333133121414131402");
        disk.defragment_full();
        // Act
        let result = disk.checksum();
        println!("{:#?}", &disk.blocks);
        // Assert
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_defragment_disk() {
        // Arrange
        let mut disk = Disk::from_string("2333133121414131402");
        // Act
        disk.defragment();
        // Assert
        assert_eq!(disk.blocks[2], 9);
        assert_eq!(disk.blocks[41], -1);
    }

    #[test]
    fn test_calculate_checksum_disk() {
        // Arrange
        let mut disk = Disk::from_string("2333133121414131402");
        disk.defragment();
        // Act
        let result = disk.checksum();
        // Assert
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_find_block_position_size_and_id() {
        // Arrange
        let disk = Disk::from_string("2333133121414131402");
        // Act
        let result = disk.find_reverse_file_block_position_and_size_by_id(41, 7);
        // Assert
        assert_eq!(result, (32, 3));
    }

    #[test]
    fn test_find_free_block_position_and_size() {
        // Arrange
        let disk = Disk::from_string("2333133121414131402");
        // Act
        let result = disk.find_free_block_position_and_size(5, 41, 2);
        // Assert
        assert_eq!(result, Some((8, 3)));
    }
}
