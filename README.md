# RESTful Rust

This project is a **RESTFul API** built with **Rust** and the **actix-web** framework.

## Endpoints

- `GET /posts` - Get a list of all posts
- `GET /posts/{id}` - Get a spesific post
- `POST /posts/` - Create a new post
- `PATCH /posts/{id}` - Update an existing post
- `DELETE /posts/{id}` - Delete a specific post

## Getting Started

1. Install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org).
2. Clone the repository to your local machine.
3. Run the command `cargo install` to install the necessary dependencies.
4. Run the command `cargo run` in the project directory to start the server.
5. Access the API endpoints using a tool such as curl.

## Code Structure

The `src` directory contains the following Rust modules:

- `error.rs`: Contains the error types and handling for the API.
- `handler.rs`: Contains the HTTP request handlers for the API endpoints.
- `main.rs`: The main entry point for the application.
- `model.rs`: Contains the data model for posts.
- `repository.rs`: Contains the SQLite database repository for storing and retrieving posts.
- `router.rs`: Contains the routing information for the API endpoints.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the terms of the MIT License. See the [LICENSE](./LICENSE) file for details.
