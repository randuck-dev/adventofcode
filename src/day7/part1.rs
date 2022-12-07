// Heavily inspired by: https://github.com/Nauss/AdventOfCode/blob/main/2022/day_7/src/main.rs

const MAX_DIR_SIZE: usize = 100000;
const TOTAL_DISK_SIZE_AVAILABLE: usize = 70000000;
const UNUSED_SPACE_NEEDED: usize = 30000000;

pub fn solve() {
    let data = include_str!("../inputs/7.txt");

    let lines = data.lines();

    let mut current: Option<usize> = None;

    // We use the file tree trick here to make it easier on using a tree type structure inside of rust.
    let mut file_tree = FileTree { files: vec![] };

    for line in lines {
        let splitted: Vec<&str> = line.split(" ").collect();
        match splitted[0] {
            // We see a command, ls or cd
            "$" => match splitted[1] {
                "cd" => match splitted[2] {
                    ".." => {
                        current = file_tree.node(current).parent;
                    }
                    _ => {
                        current = file_tree.insert(0, current);
                    }
                },
                _ => {}
            },
            // we do not care about the directory itself
            "dir" => {}
            // We see a file, lets take it size
            _ => _ = file_tree.insert(splitted[0].parse::<usize>().unwrap(), current),
        }
    }
    let directories: Vec<&TreeNode> = file_tree
        .files
        .iter()
        .filter(|node| node.is_directory())
        .collect();

    let part1: usize = directories
        .iter()
        .filter(|node| node.value <= MAX_DIR_SIZE)
        .map(|node| node.value)
        .sum();

    let to_delete_size =
        UNUSED_SPACE_NEEDED - (TOTAL_DISK_SIZE_AVAILABLE - file_tree.files[0].value);
    let part2: usize = directories
        .iter()
        .filter(|node| node.value >= to_delete_size)
        .map(|node| node.value)
        .min()
        .unwrap();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

struct FileTree {
    files: Vec<TreeNode>,
}

impl FileTree {
    fn insert(&mut self, value: usize, parent: Option<usize>) -> Option<usize> {
        // Get the next index
        let index = self.files.len();

        if let Some(parent) = parent {
            self.files[parent].children.push(index);
            self.update_size(Some(parent), value);
        }

        self.files.push(TreeNode {
            value,
            parent,
            children: vec![],
        });

        Some(index)
    }

    fn node(&self, index: Option<usize>) -> &TreeNode {
        match index {
            Some(index) => &self.files[index],
            None => panic!("Trying to access unknown treenode"),
        }
    }

    fn update_size(&mut self, index: Option<usize>, size: usize) {
        let mut current = index;
        while let Some(index) = current {
            let node = &mut self.files[index];
            node.value += size;
            current = node.parent;
        }
    }
}

#[derive(PartialEq)]
struct TreeNode {
    value: usize,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl TreeNode {
    fn is_directory(&self) -> bool {
        self.children.len() > 0
    }
}
