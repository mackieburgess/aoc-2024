use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct File {
    file_id: usize,
    size: usize,
    starts_at: usize
}

struct Slot {
    space: usize,
    starts_at: usize
}

fn compressed_checksum() -> usize {
    if let Some(input) = fs::read_to_string("data/9.input").ok() {
        let disk_map = input
            .trim()
            .chars()
            .filter_map(|c| c.to_string().parse::<usize>().ok())
            .collect::<Vec<_>>();

        let mut files = disk_map
            .clone()
            .into_iter()
            .step_by(2)
            .enumerate()
            .map(|(file_id, size)| {
                File {
                    starts_at: disk_map.iter().take(file_id * 2).sum(),
                    file_id,
                    size
                }
            })
            .collect::<VecDeque<File>>();

        let mut slots = disk_map
            .clone()
            .into_iter()
            .skip(1)
            .step_by(2)
            .enumerate()
            .map(|(index, space)| {
                Slot {
                    starts_at: disk_map.iter().take(index * 2 + 1).sum(),
                    space
                }
            })
            .collect::<VecDeque<Slot>>();

        while let Some(slot) = slots.pop_front() {
            if files.back().unwrap().starts_at < slot.starts_at {
                break;
            }

            // Put as much file into the slot as possible
            let file = files.pop_back().unwrap();
            if slot.space > file.size {
                let new_file = File {
                    file_id: file.file_id,
                    size: file.size,
                    starts_at: slot.starts_at
                };

                let new_slot = Slot {
                    space: slot.space - file.size,
                    starts_at: slot.starts_at + file.size
                };

                // Doesn't matter if order in the VecDeque is lost, I just never want to see
                // this file again in the loop.
                files.push_front(new_file);
                slots.push_front(new_slot);
            } else {
                let new_file = File {
                    file_id: file.file_id,
                    size: slot.space,
                    starts_at: slot.starts_at
                };
                files.push_front(new_file);

                if file.size > slot.space {
                    let remnants = File {
                        file_id: file.file_id,
                        size: file.size - slot.space,
                        starts_at: file.starts_at
                    };
                    files.push_back(remnants);
                }
            }
        }

        return files
            .into_iter()
            .map(|file| {
                (file.starts_at..(file.starts_at + file.size))
                    .map(|position| position * file.file_id)
                    .sum::<usize>()
            })
            .sum();
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", compressed_checksum());
}
