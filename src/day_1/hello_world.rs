fn user_description(old_name: &str, age: i32) {
    let user_description = if age < 18 {
        "young"
    } else if age <= 70 {
        "adult"
    } else {
        "old"
    };

    println!(
        "Hi, my name was {old_name}. I used to be a {user_description} human, but today I am a robot called Matt007 and I am 1500 years old."
    );
}

pub fn hello_world() {
    let age: i32 = 15;
    let old_name: &str = "Matias Torres";

    user_description(old_name, age);
}
