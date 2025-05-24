use crate::task::Task;
use crate::storage::{load_todos, save_todos};
use std::io::{self, Write};

pub fn run() {
    let mut todos = load_todos();

    loop {
        println!("\n==== TODO LIST ====");
        for (i, task) in todos.iter().enumerate() {
            println!("{}. {}", i + 1, task.display());
        }

        println!("\nMenu:");
        println!("1. Tambah Todo");
        println!("2. Hapus Todo");
        println!("3. Edit Todo");
        println!("4. Tandai Selesai/Belum");
        println!("5. Keluar");

        let choice = input("Pilih opsi (1-5):");

        match choice.as_str() {
            "1" => {
                let name = input("Masukkan tugas baru:");
                let deadline_input = input("Masukkan deadline (dd-mm-yyyy), atau kosongkan:");
                let deadline = if deadline_input.trim().is_empty() {
                    None
                } else {
                    Some(deadline_input)
                };
                todos.push(Task::new(&name, deadline));
                save_todos(&todos);
            }
            "2" => {
                if let Some(i) = select_index(&todos, "Nomor tugas yang ingin dihapus:") {
                    todos.remove(i);
                    save_todos(&todos);
                }
            }
            "3" => {
                if let Some(i) = select_index(&todos, "Nomor tugas yang ingin diedit:") {
                    println!("Tugas saat ini: {}", todos[i].name);
                    println!("1. Ganti penuh");
                    println!("2. Tambah teks");

                    let sub = input("Pilih opsi edit (1-2):");

                    match sub.as_str() {
                        "1" => {
                            let baru = input("Masukkan teks pengganti:");
                            todos[i].name = baru;
                            save_todos(&todos);
                        }
                        "2" => {
                            let tambahan = input("Tambahkan teks:");
                            todos[i].name = format!("{} {}", todos[i].name, tambahan.trim());
                            save_todos(&todos);
                        }
                        _ => println!("Opsi edit tidak dikenali."),
                    }
                }
            }
            "4" => {
                if let Some(i) = select_index(&todos, "Nomor tugas yang ingin ditandai selesai/belum:") {
                    todos[i].done = !todos[i].done;
                    save_todos(&todos);
                }
            }
            "5" => {
                println!("Sampai jumpa, sayang ❤️");
                break;
            }
            _ => println!("Opsi tidak dikenali."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn select_index(todos: &[Task], prompt: &str) -> Option<usize> {
    let input = input(prompt);
    match input.trim().parse::<usize>() {
        Ok(i) if i > 0 && i <= todos.len() => Some(i - 1),
        _ => {
            println!("Nomor tidak valid.");
            None
        }
    }
}

