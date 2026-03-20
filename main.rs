use std::io::{self, Write};
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // تعريف الألوان
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    println!("{}--- 🛡️ نظام Ranger الذكي v2.0 ---{}", cyan, reset);
    print!("🔑 مفتاح الدخول: ");
    io::stdout().flush().unwrap();

    let mut pass = String::new();
    io::stdin().read_line(&mut pass).expect("خطأ");

    if pass.trim() == "Ranger2024" {
        println!("{}✅ أهلاً بك يا محمد في قبوك الخاص.{}", green, reset);
        println!("{}كيف تشعر اليوم؟ (أنا هنا لأسمعك...){}", yellow, reset);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("خطأ");
        let mood = input.trim().to_lowercase();

        // تحليل ذكي بسيط للمشاعر
        let response = if mood.contains("سعيد") || mood.contains("خير") || mood.contains("تمام") {
            "🌟 رائع! أتمنى أن تدوم هذه الطاقة الإيجابية."
        } else if mood.contains("حزين") || mood.contains("متعب") || mood.contains("ضيق") {
            "🌿 لا بأس يا محمد، غداً سيكون أجمل. لقد حفظت سرك بأمان."
        } else {
            "🤖 فهمت.. تم تدوين مشاعرك في الذاكرة المشفرة."
        };

        println!("{}{}{}", cyan, response, reset);

        // التشفير والحفظ مع التوقيت
        let encrypted: String = mood.chars().map(|c| ((c as u8) + 1) as char).collect();
        let mut file = OpenOptions::new().create(true).append(true).open("vault_logs.txt").expect("خطأ");
        writeln!(file, "--- دخول جديد لـ محمد ---").expect("خطأ");
        writeln!(file, "المشاعر المشفرة: {}", encrypted).expect("خطأ");
        
    } else {
        println!("\x1b[31m❌ محاولة دخول غير مصرح بها! تم قفل النظام.\x1b[0m");
    }
}

