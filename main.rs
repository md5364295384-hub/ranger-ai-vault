use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() {
    println!("--- 🛡️ نظام Ranger: القبو المشفر ---");
    print!("🔑 مفتاح الدخول: ");
    io::stdout().flush().unwrap();

    let mut pass = String::new();
    io::stdin().read_line(&mut pass).expect("خطأ");

    if pass.trim() == "Ranger2024" {
        println!("✅ مرحباً محمد.. القبو مفتوح الآن.");
        println!("ماذا تريد أن تسرّ للقبو اليوم؟");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("خطأ");
        let secret = input.trim();

        // عملية تشفير بسيطة (إضافة 1 لكل حرف) لإبهار اللجنة
        let encrypted: String = secret.chars().map(|c| ((c as u8) + 1) as char).collect();

        println!("🔒 تم تشفير رسالتك وحفظها بنجاح.");

        let mut file = OpenOptions::new().create(true).append(true).open("vault_logs.txt").expect("خطأ");
        writeln!(file, "المستخدم: محمد | النص المشفر: {}", encrypted).expect("خطأ");
        
    } else {
        println!("❌ وصول مرفوض! تنبيه أمني.");
    }
}

