use std::io;

fn main() {
    println!("--- نظام Ranger للتحليل الذكي ---");
    println!("كيف حالك اليوم؟ (اكتب أي شيء وسأحلل كلامك)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("خطأ في القراءة");

    let text = input.trim().to_lowercase();

    if text.contains("سعيد") || text.contains("ممتاز") || text.contains("بخير") {
        println!("🤖 المساعد الذكي: يبدو أنك في حالة معنوية رائعة! استمر يا بطل.");
    } else if text.contains("حزين") || text.contains("تعبان") || text.contains("صعب") {
        println!("🤖 المساعد الذكي: أنا هنا لأدعمك. تذكر أن الصعاب هي من تصنع المبرمجين الأقوياء.");
    } else {
        println!("🤖 المساعد الذكي: شكراً لمشاركتي هذا. أنا أتعلم من كلماتك كل يوم.");
    }
}

