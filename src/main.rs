mod command;
use command::command::*;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("=== DEMO CHƯƠNG TRÌNH RUST ===");

    // 1. Tạo các file dữ liệu mẫu để test tính năng Search
    setup_dummy_files();

    // 2. Khởi tạo bộ lưu trữ PointerStorage
    let mut storage = PointerStorage::new();

    // 3. Khởi tạo dữ liệu file muốn mở (DataPointer)
    let pointer1 = DataPointer {
        name: String::from("file_log"),
        file: MyFile {
            file_path: String::from("log.txt"),
        },
        content: Text::Content(String::from("Nội dung file log")),
    };

    let pointer2 = DataPointer {
        name: String::from("file_config"),
        file: MyFile {
            file_path: String::from("config.txt"),
        },
        content: Text::NoneAtAll,
    };

    // ==========================================
    // DEMO 1: Chạy lệnh Test
    // ==========================================
    println!("\n--- 1. Thực thi Argument::Test ---");
    let mut test_cmd = Command {
        argurments: vec![Argument::Test],
        pattern: String::new(),
        data_pointer_map: vec![],
    };
    test_cmd.execute(&mut storage);

    // ==========================================
    // DEMO 2: Thêm file vào Storage (OpenFile)
    // ==========================================
    println!("\n--- 2. Thực thi Argument::OpenFile (Mở file lần 1) ---");
    let mut open_cmd = Command {
        argurments: vec![Argument::OpenFile],
        pattern: String::new(),
        data_pointer_map: vec![pointer1.clone(), pointer2.clone()],
    };
    open_cmd.execute(&mut storage);
    println!(
        "Số lượng file trong storage hiện tại: {}",
        storage.pointer_list.len()
    );

    // Thử mở lại đúng file đó để test hàm check_duplicate
    println!("\n--- 3. Thử mở lại file đã tồn tại (Test Duplicate) ---");
    open_cmd.execute(&mut storage);

    // ==========================================
    // DEMO 3: Tìm kiếm từ khóa trong file (Search)
    // ==========================================
    println!("\n--- 4. Thực thi Argument::Search (Tìm từ khóa 'Rust') ---");
    let mut search_cmd = Command {
        argurments: vec![Argument::Search],
        pattern: String::from("Rust"), // Tìm các dòng chứa chữ "Rust"
        data_pointer_map: vec![pointer1, pointer2],
    };
    search_cmd.execute(&mut storage);

    // Dọn dẹp các file mẫu sau khi chạy xong
    clean_dummy_files();
}

// --- Hàm phụ trợ để tạo file mẫu trên ổ đĩa ---
fn setup_dummy_files() {
    if let Ok(mut f1) = File::create("log.txt") {
        let _ = writeln!(f1, "Dòng 0: Xin chào thế giới");
        let _ = writeln!(f1, "Dòng 1: Lập trình Rust rất thú vị");
        let _ = writeln!(f1, "Dòng 2: Bài học Rust nâng cao");
    }

    if let Ok(mut f2) = File::create("config.txt") {
        let _ = writeln!(f2, "Dòng 0: env=production");
        let _ = writeln!(f2, "Dòng 1: language=Rust");
    }
}

// --- Hàm dọn dẹp file sau khi test ---
fn clean_dummy_files() {
    let _ = std::fs::remove_file("log.txt");
    let _ = std::fs::remove_file("config.txt");
}
