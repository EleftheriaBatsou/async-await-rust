### ow to make an asynchronous GET request using `async` and `await` 🦀

The `main` function is the entry point of the program, marked with the `#[tokio::main]` attribute. This attribute allows the program to use Tokio's runtime, enabling asynchronous execution.

The function is defined as asynchronous (`async fn main() -> Result<()>`), indicating that it can perform asynchronous operations using `async` and `await`.

`reqwest::get("http://httpbin.org/get").await?` sends an asynchronous GET request to the specified URL (`http://httpbin.org/get`) using the `reqwest` crate. 

The `await` keyword is used here to asynchronously wait for the completion of the request. 

The `?` operator is used to handle any potential errors that may occur during the request.

After receiving the response, the status code and headers of the response are printed to the console.

`res.text().await?` asynchronously reads the response body as a string. The `await` keyword is used here to asynchronously wait for the completion of reading the response body. The `?` operator is used to handle any potential errors that may occur during reading.

The response body is then printed to the console.

`Ok(())` indicates that the program executed successfully and returns an empty `Result`.

**Overall, this code demonstrates how to:**

✓ make an asynchronous GET request using the `reqwest` crate in Rust

✓ handle the response asynchronously using `async` and `await`

✓ print the status code, headers, and body of the response

✓ additionally, it utilizes the `error_chain` crate for error handling

