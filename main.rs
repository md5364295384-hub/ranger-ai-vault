use std::io::{self, Write, BufReader, BufRead};
use std::fs::{OpenOptions, File};

fn main() {
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    println!("{}--- 🛡️ نظام Ranger الذكي v3.0 ---{}", cyan, reset);
    print!("🔑 مفتاح الدخول: ");
    io::stdout().flush().unwrap();

    let mut pass = String::new();
    io::stdin().read_line(&mut pass).expect("خطأ");

    if pass.trim() == "Ranger2024" {
        // ميزة الفوز: قراءة عدد الأسطر لمعرفة عدد الأسرار
        let count = if let Ok(f) = File::open("vault_logs.txt") {
            BufReader::new(f).lines().count() / 2 // نقسم على 2 لأن كل دخول يسجل سطرين
        } else { 0 };

        println!("{}✅ أهلاً محمد. لديك حالياً {} أسرار محفوظة في القبو.{}", green, count, reset);
        println!("{}كيف حالك اليوم؟{}", yellow, reset);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("خطأ");
        let mood = input.trim();

        let response = if mood.contains("سعيد") || mood.contains("خير") {
            "🌟 رائع! طاقتك ملهمة جداً."
        } else {
            "🤖 تم الحفظ بنجاح. تذكر أنني هنا دائماً."
        };

        println!("{}{}{}", cyan, response, reset);

        let encrypted: String = mood.chars().map(|c| ((c as u8) + 1) as char).collect();
        let mut file = OpenOptions::new().create(true).append(true).open("vault_logs.txt").expect("خطأ");
        writeln!(file, "تاريخ: محمد | النص المشفر: {}", encrypted).expect("خطأ");
        
    } else {
        println!("\x1b[31m❌ وصول مرفوض!\x1b[0m");
    }
}

