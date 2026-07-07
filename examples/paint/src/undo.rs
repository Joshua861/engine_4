use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sge::flate2;
use std::io::{Read, Write};

pub struct UndoCache {
    back: Vec<Vec<u8>>,
    forward: Vec<Vec<u8>>,
    last_buffer: Vec<u8>,
}

impl UndoCache {
    pub fn new() -> Self {
        Self {
            back: vec![],
            forward: vec![],
            last_buffer: vec![],
        }
    }

    pub fn undo_buffer(&mut self, current_buffer: &mut [u8]) {
        if let Some(compressed_diff) = self.back.pop() {
            let mut decoder = ZlibDecoder::new(&compressed_diff[..]);
            let mut diff = Vec::new();
            decoder.read_to_end(&mut diff).unwrap();

            self.forward.push(compressed_diff);

            for (i, &diff_byte) in diff.iter().enumerate() {
                if i < current_buffer.len() {
                    current_buffer[i] ^= diff_byte;
                }
            }

            self.last_buffer.copy_from_slice(current_buffer);
        }
    }

    pub fn redo_buffer(&mut self, current_buffer: &mut [u8]) {
        if let Some(compressed_diff) = self.forward.pop() {
            let mut decoder = ZlibDecoder::new(&compressed_diff[..]);
            let mut diff = Vec::new();
            decoder.read_to_end(&mut diff).unwrap();

            self.back.push(compressed_diff);

            for (i, &diff_byte) in diff.iter().enumerate() {
                if i < current_buffer.len() {
                    current_buffer[i] ^= diff_byte;
                }
            }

            self.last_buffer.copy_from_slice(current_buffer);
        }
    }

    pub fn handle_buffer_update(&mut self, current_buffer: &[u8]) {
        if self.last_buffer.is_empty() {
            self.last_buffer = current_buffer.to_vec();
            return;
        }

        if self.last_buffer == current_buffer {
            return;
        }

        assert_eq!(
            self.last_buffer.len(),
            current_buffer.len(),
            "Buffer size changed unexpectedly"
        );

        self.forward.clear();

        let diff: Vec<u8> = current_buffer
            .iter()
            .zip(self.last_buffer.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(&diff).unwrap();
        let bytes = e.finish().unwrap();

        self.back.push(bytes);
        self.last_buffer = current_buffer.to_vec();

        eprintln!(
            "Cache size (kB): {}",
            self.back.iter().map(|v| v.len()).sum::<usize>() as f32 / 8.0 / 1024.0
        );
    }
}
