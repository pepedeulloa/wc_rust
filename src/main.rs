use wc_rust::config::{ get_args, is_default, output, run_file, total, Cli, FileCount};

fn main() {
    let Cli{files, bytes, lines, chars, words} = get_args().unwrap();

    let num_files: usize = files.len();
    let mut file_count_vec: Vec<FileCount> = Vec::new();
    
    if num_files > 1 {
        for f in files {
            let file_count: FileCount = run_file(f.clone(), is_default(bytes, chars, lines, words), bytes, lines, chars, words).unwrap(); 
            output(&file_count, f.clone().to_str().unwrap(), is_default(bytes, chars, lines, words), bytes, lines, chars, words);
            file_count_vec.push(file_count);
        }
        let total: FileCount = total(file_count_vec).unwrap();
        output(&total, "total", is_default(bytes, chars, lines, words), bytes, lines, chars, words)
    } else {
        let file_count: FileCount = run_file(files[0].clone(), is_default(bytes, chars, lines, words), bytes, lines, chars, words).unwrap();
        output(&file_count, files[0].clone().to_str().unwrap(), is_default(bytes, chars, lines, words), bytes, lines, chars, words);
    }
}