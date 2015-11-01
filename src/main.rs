#![feature(plugin)]
#![feature(unsafe_destructor)]
#![plugin(concat_bytes)]

#[macro_use] extern crate webplatform;
extern crate libc;

use std::borrow::ToOwned;

fn main() {
    let document = webplatform::init();
    {
        let body = document.element_query("body").unwrap();

        let hr = document.element_create("hr").unwrap();
        body.append(&hr);

        body.html_prepend("<h1>HELLO FROM RUST</h1>");
        body.html_append("<button>CLICK ME</button>");

        let mut button = document.element_query("button").unwrap();

        let bodyref = body.root_ref();
        let bodyref2 = body.root_ref();
        button.on("click", move |_| {
            bodyref2.prop_set_str("bgColor", "blue");
        });

        let jquery = webplatform::JQuery::new();

        jquery.ajax("/webplatform.html", move |data| {
            println!("ajax executed!, data: {:?}", data);
        });

        webplatform::SessionStorageInterface.set("start", "0");

        js! {
            br#"
                var start = sessionStorage.getItem('start');
                console.log({start: start});
            "#
        };

        println!("This should be blue: {:?}", bodyref.prop_get_str("bgColor"));
        println!("Width?: {:?}", bodyref.prop_get_i32("clientWidth"));
        println!("Timestamp: {:?}", webplatform::Date::now());

        webplatform::spin();
    }

    println!("NO CALLING ME.");
}
