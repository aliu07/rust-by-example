use trpl::{Either, Html};

// Each await point—that is, every place where the code uses the await keyword—represents
// a place where control is handed back to the runtime. To make that work, Rust needs to
// keep track of the state involved in the async block so that the runtime can kick off some
// other work and then come back when it’s ready to try advancing the first one again.
// This is an invisible state machine.
//
// Writing the code to transition between each state by hand would be tedious and error-prone,
// however, especially when you need to add more functionality and more states to the code
// later. Fortunately, the Rust compiler creates and manages the state machine data structures
// for async code automatically. The normal borrowing and ownership rules around data structures
// all still apply, and happily, the compiler also handles checking those for us and provides
// useful error messages.
//
// Ultimately, something has to execute this state machine, and that something is a runtime.
// (This is why you may come across references to executors when looking into runtimes: an
// executor is the part of a runtime responsible for executing the async code.)
//
// Now you can see why the compiler stopped us from making main itself an async. If main were
// an async function, something else would need to manage the state machine for whatever future
// main returned, but main is the starting point for the program! Instead, we called the trpl::run
// function in main to set up a runtime and run the future returned by the async block until it
// is done.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

// When Rust sees a block marked with the async keyword, it compiles it
// into a unique, anonymous data type that implements the Future trait.
// When Rust sees a function marked with async, it compiles it into a
// non-async function whose body is an async block. An async function’s
// return type is the type of the anonymous data type the compiler creates
// for that async block.
//
// Thus, writing async fn is equivalent to writing a function that returns
// a future of the return type. To the compiler, a function definition such
// as the async fn page_title is equivalent to a non-async function defined
// like this:
//
// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         let title = Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//
//         (url, title)
//     }
// }
async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let text = response.text().await;
    // OR... using chaining
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}
