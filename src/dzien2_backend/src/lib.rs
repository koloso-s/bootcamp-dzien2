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
}