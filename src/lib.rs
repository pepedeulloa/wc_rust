pub mod config {
 use std::{ error::Error, fs::File, io::{BufReader, BufRead}, path::PathBuf };

 use clap::Parser;
 
 #[derive(Parser,Debug)]
 #[command(author, version, about, long_about = None)]
 pub struct Cli {
  /// Path of the file to read
  #[arg(id="FILE")]
  pub files: Vec<PathBuf>,
 
  /// Print bytes count of the file
  #[arg(short='c', long="bytes", default_value_t=false)]
  pub bytes: bool,
 
  /// Print lines count of the file
  #[arg(short='l', long="lines", default_value_t=false)]
  pub lines: bool,
 
  /// Print character count of the file
  #[arg(short='m', long="chars", default_value_t=false)]
  pub chars: bool,
 
  /// Print words count of the file
  #[arg(short='w', long="words", default_value_t=false)]
  pub words: bool,
 
 }

	pub fn is_default(bytes: bool, chars: bool, lines: bool, words: bool) -> bool {
		if !bytes && !chars && !lines && !words {
			true
		} else {
			false
		}
	}
 
 pub struct FileCount {
  pub bytes: usize,
 
  pub lines: usize,
 
  pub chars: usize,
 
  pub words: usize,
 }

	impl FileCount {
		pub fn new() -> Self {
			FileCount {
				bytes: 0, 
				lines: 0, 
				chars: 0, 
				words: 0
			}
		}

		pub fn increment_counters(&mut self, file_count: FileCount) {
			self.bytes += file_count.bytes;
			self.lines += file_count.lines;
			self.chars += file_count.chars;
			self.words += file_count.words;
		}
	}
 
 pub fn get_args() -> Result<Cli, Box<dyn Error>> {
  let cli = Cli::parse();
  Ok(cli)
 }
 
 fn open(filename: PathBuf) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
  let file = File::open(filename)?;
  let reader = BufReader::new(file);
  
  Ok(reader)
 }
 
	fn count_bytes (line: &String) -> Result<usize, Box<dyn std::error::Error>>  {
  let count = line.bytes().count();
  Ok(count)
 }

 fn count_chars (line: &String) -> Result<usize, Box<dyn std::error::Error>> {
  let count = line.chars().count();
  Ok(count)
 }

 fn count_words(line: &String) -> Result<usize, Box<dyn std::error::Error>> {
  Ok(line.split_whitespace().count())
 }

 pub fn run_file(file: PathBuf, default: bool, bytes: bool, lines: bool, chars: bool, words: bool ) -> Result<FileCount,Box <dyn std::error::Error>> {
		let reader = open(file).unwrap();
		
		let mut file_count = FileCount{
			bytes : 0,
			lines : 0,
			chars : 0,
			words : 0
		};

  for line in reader.lines(){

			if default {
				file_count.lines += 1;
				file_count.words += count_words(line.as_ref().unwrap()).unwrap();
				file_count.bytes += count_bytes(line.as_ref().unwrap()).unwrap();
			}

   if bytes {
    file_count.bytes += count_bytes(line.as_ref().unwrap()).unwrap();
   }
   if chars {
    file_count.chars += count_chars(line.as_ref().unwrap()).unwrap();
   }
   if words {
    file_count.words += count_words(line.as_ref().unwrap()).unwrap();
   }
   if lines {
    file_count.lines += 1;
   }
  }
		Ok(file_count)
 }

	pub fn output(f: &FileCount, name: &str, default: bool, bytes: bool, lines: bool, chars: bool, words: bool) {
		if default {
			print!("\t{}\t{}\t{}\t{}\n", f.lines, f.words, f.bytes, name);
		}
		if lines {
			print!("\t{}", f.lines);
		}
		if bytes {
			print!("\t{}", f.bytes);
		}
		if chars {
			print!("\t{}", f.chars);
		}
		if words {
			print!("\t{}", f.words);
		}
		if !default {
			print!("\t{}\n", name);
		}
	}

	pub fn total(vector: Vec<FileCount>) -> Result<FileCount, Box <dyn std::error::Error>> {
		let mut file_count = FileCount{
			bytes : 0,
   lines : 0,
   chars : 0,
   words : 0
		};
		for el in vector  {
			file_count.increment_counters(el)
		}
		Ok(file_count)
	}
} 