use std::{fs::File, io::{BufReader, BufWriter, Read, Seek, Write}, os::windows::fs::FileExt};
use crate::{traits::Seekable, utils::redact_comment_block::redact_comment_block};

const PAGE_START_TAG: &[u8; 6] = b"<page>";
const PAGE_END_TAG: &[u8; 7] = b"</page>";


pub fn dump_to_filtered(dump: &str, out: &str, search_headers: &[&str]) -> Result<(), String> {

    let dump = File::options()
        .create_new(false)
        .create(false)
        .read(true)
        .open(dump)
        .expect("dump file");

    let dump_size = dump.metadata().expect("meta").len();
    let mut reader = BufReader::with_capacity(1024*1024*500, dump);


    let out_file = File::options()
        .read(true)
        .create_new(true)
        .append(true)
        .open(out)
        .expect("out file");
    let mut out_buffer = BufWriter::new(out_file);


    // Buffer to hold the data read from the file (Have to initialize like this because otherwise it will overflow stack before being moved over)
    let proto_buf: Vec<u8> = vec![
        0; // Fill
        1024 // kilobyte
        * 1024 // megabyte
        * 500 //
    ];
    let mut active_buffer: Box<[u8]> = proto_buf.into_boxed_slice();
    let mut current: usize = 0;

    loop {
        let bytes_read = reader
            .read(&mut active_buffer)
            .expect("read from file");

        if bytes_read == 0 {
            out_buffer.flush().expect("Flush");
            break
        }
        current += bytes_read;

        println!("{current}/{total}mb", 
            current = current / 1024 / 1024,
            total = dump_size / 1024 / 1024
        );

        let page_start_tag_occurrences = active_buffer.as_ref().find_all(PAGE_START_TAG);

        for page_start_pos in page_start_tag_occurrences {
            if let Some(page_end_pos) = &active_buffer[page_start_pos..].as_ref().find(PAGE_END_TAG) {
                let page_end_pos = page_start_pos + page_end_pos + PAGE_END_TAG.len();
                let mut page_content = &mut active_buffer[page_start_pos..page_end_pos];
    
                if search_headers.iter()
                    .any(|h| page_content.as_ref().find(h.as_bytes()).is_some()) {
    
                    redact_comment_block(&mut page_content);
    
                    out_buffer
                        .write(b"===================================================\n")
                        .expect("File write");
                    out_buffer.write(page_content).expect("File write");
                    out_buffer.write(b"\n").expect("File write");
                }
            }
        }

        out_buffer.flush().expect("Flush");
    }

    println!("Complete!");

    Ok(())
}

