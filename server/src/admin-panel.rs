struct Admin {}

pub trait admin_scaner_base {
    fn show(); //Показать результаты сканирования
    fn remove(); //Очистить результаты
    fn analyse(); //Проанализировать результаты
}

pub trait admin_scaner {
    fn add(); //Добавить сканер
    fn update_scaner_info(); //Изменить информацию о сканере
    fn take_result(); //Забрать результаты сканера и сохранить у себя
    fn show_scaner_info(); //Показать информацию о сканере
    fn update_database(); //Обновить информацию у сканера
    fn remove_log(); //Очистить историю у Сканера
}

impl admin_scaner_base for Admin {}
