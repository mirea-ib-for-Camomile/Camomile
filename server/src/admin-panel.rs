struct Admin{}

pub trait admin_data_base{
    fn add();
    fn remove();
    fn change();
}

impl admin_data_base for Admin{
    
}