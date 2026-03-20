use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() {
    println!("--- قبو Ranger الذكي ---");
    println!("أهلاً بك يا محمد، كيف تشعر اليوم؟");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("خطأ في القراءة");
    let text = input.trim();

    let response = if text.contains("سعيد") || text.contains("بخير") {
        "🤖: هذا رائع! طاقتك الإيجابية ملهمة."
    } else if text.contains("تعبان") || text.contains("صعب") {
        "🤖: خذ قسطاً من الراحة، تذكر أنك مبرمج بطل."
    } else {
        "🤖: شكراً لمشاركتي، سأحفظ هذا في ذاكرتي."
    };

    println!("{}", response);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("vault_logs.txt")
        .expect("لا يمكن فتح ملف الذاكرة");

    writeln!(file, "محمد قال: {} | الرد كان: {}", text, response).expect("فشل الحفظ");
    println!("🔒 (تم تشفير وحفظ المدخلات في ملف الذاكرة السري)");
}

