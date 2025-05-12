use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n==== Todo List ====");
        for (i, task) in todos.iter().enumerate() {
            println!("{}. {}", i + 1, task);

        }

        println!("\nMenu:");
        println!("1. Tambah Todo");
        println!("2. Hapus Todo");
        println!("3. Keluar");

        print!("Pilih opsi (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "1" => {
                print!("masukkan tugas baru: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todos.push(task.trim().to_string());
            }
            "2" => {
                print!("masukkan nomor tugas yang ingin dihapus: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= todos.len() {
                        todos.remove(i - 1);
                    } else {
                        println!("Nomor tidak valid.");

                    }
                } else {
                    println!("Input bukan angka.");
                }

            }
            "3" => {
                println!("Sampai jumpa, Tuan!");
                break;
            }
            _ => println!("Opsi tidak dikenali."),
        }
    }
}

