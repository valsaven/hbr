use std::io::stdin;

mod sub;
use sub::{open_file, get_user_data, write_data_to_file};

fn main() {
    let mut is_loop_continue = String::new();

    /* Запускаем бесконечный цикл опроса */
    loop {
        // Проверяем нужно ли продолжать цикл
        if is_loop_continue.chars().count() > 1 {
            println!("Data saved. Power off.");
            // Выход с цикла
            break;
        }

        // Запрашиваем ввод ссылки, заголовка статьи и мнения по статье
        let link = get_user_data("link");
        let title = get_user_data("title");
        let opinion = get_user_data("opinion");

        // В созданный/существующий файл добавляет \n и добавляем всё выше указанное
        let file = open_file();
        write_data_to_file(file, link, title, opinion);

        // Запрос на повтор цикла
        println!("Continue?");
        println!("y (Enter) / n (type something)");
        // stop = &mut String::new();


        is_loop_continue = String::new();
        stdin().read_line(&mut is_loop_continue)
            .ok()
            .expect("Failed to read line");


        /* Конец бесконечного цикла */
    }
}