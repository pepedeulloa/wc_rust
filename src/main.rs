use wc_rust::config::{ Cli, get_args, open, run_file, FileCount};

fn main() {
    let Cli{files, bytes, lines, chars, words} = get_args().unwrap();

    for f in files {
        let name = f.clone();
        let reader = open(f).unwrap();
        let FileCount {bytes, lines, words, ..}: FileCount = run_file(reader, bytes, lines, chars, words).unwrap();
        println!("\tlines:{:?}\twords:{:?}\tbytes:{:?} {:?}", lines, words, bytes, name )
    }

}