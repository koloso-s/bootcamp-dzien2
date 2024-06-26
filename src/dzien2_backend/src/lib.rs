use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn dodaj_wpis(wpis: String){
    // let wpis2: String = wpis;
    // let wpis3: String = wpis;
    WPISY.with(|wpisy|{
        wpisy.borrow_mut().push(wpis)
    });

    // let zmienna:String = String::from("siema");
    // let zmienna2 = &zmienna;
    // zmienna2.as_mut();
    // let zmienna3 = zmienna;
}

#[ic_cdk::query]
fn odczytaj_wpisy() ->Vec<String>{
    WPISY.with(|wpisy|{
        wpisy.borrow().clone()
    })
}

#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize){
    WPISY.with(|wpisy|{
        wpisy.borrow_mut().remove(id_wpisu)
    });
}

#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize,nowy_wpis: String){
    WPISY.with(|wpisy|{
        let mut binding = wpisy.borrow_mut();
        let wpis = binding.get_mut(id_wpisu);
        let stary_wpis = wpis.unwrap();
        *stary_wpis = nowy_wpis;
    });
}