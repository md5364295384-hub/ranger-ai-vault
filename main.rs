use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() {
    println!("--- 🛡️ قبو Ranger المحمي ---");
    print!("🔑 أدخل كلمة المرور: ");
    io::stdout().flush().unwrap();

    let mut pass = String::new();
    io::stdin().read_line(&mut pass).expect("خطأ في القراءة");

    if pass.trim() == "Ranger2024" {
        println!("✅ أهلاً بك يا محمد. كيف حالك اليوم؟");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("خطأ في القراءة");
        
        let response = "🤖 تم تسجيل مشاعرك في القبو السري.";
        println!("{}", response);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("vault_logs.txt")
            .expect("خطأ في فتح الملف");
            
        writeln!(file, "محمد قال: {} | الرد: {}", input.trim(), response).expect("خطأ في الحفظ");
    } else {
        println!("❌ كلمة مرور خاطئة! تم حظر الدخول.");
    }
}

