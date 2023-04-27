# RESTful Rust

This repository contains the source code for a Rust-based REST API that uses Actix-web as its framework. The API provides CRUD functionality for posts stored in a SQLite database.

## Getting Started

1. Install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org).
2. Clone the repository to your local machine.
3. Run the command `cargo run` in the project directory to start the server.
4. Access the API endpoints using a tool such as [Postman](https://www.postman.com) or cURL.

## API Endpoints

### `GET /posts`

Returns a list of all the blog posts.

#### Response

```txt
HTTP/1.1 200 OK
Content-Type: application/json

[
    {
        "id": 1,
        "title": "First post",
        "content": "This is my first blog post."
    },
    {
        "id": 2,
        "title": "Second post",
        "content": "This is my second blog post."
    },
    ...
]
```

### `GET /posts/{id}`

Returns the blog post with the specified ID.

#### Parameters

| Parameter | Type | Description                          |
| :-------- | :--- | :----------------------------------- |
| id        | int  | The ID of the blog post to retrieve. |

#### Response

```txt
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "title": "First post",
    "content": "This is my first blog post."
}
```

### `POST /posts/`

Creates a new blog post.

#### Request Body

| Field | Type   | Description                       |
| :---- | :----- | :-------------------------------- |
| title | string | The title of the new blog post.   |
| body  | string | The content of the new blog post. |

####  Example

```txt
POST /posts
Content-Type: application/json

{
    "title": "Third post",
    "content": "This is my third blog post."
}
```

#### Response

```txt
HTTP/1.1 200 OK
```

### `PATCH /posts/{id}`

Updates an existing blog post with the specified ID.

#### Parameters

| Parameter | Type | Description                       |
| :-------- | :--- | :-------------------------------- |
| id        | int  | The ID of the blog post to update |

#### Request Body

| Field | Type   | Description                       |
| :---- | :----- | :-------------------------------- |
| title | string | The new title for the blog post.  |
| body  | string | The new content of the blog post. |

#### Example

```txt
PATCH /posts/1
Content-Type: application/json

{
    "title": "Updated first post",
    "content": "This is my first blog post, updated."
}
```

#### Response

```txt
HTTP/1.1 200 OK
```

### `DELETE /posts/{id}`

Deletes the blog post with the specified ID.

#### Parameters

| Parameter | Type | Description                        |
| :-------- | :--- | :--------------------------------- |
| id        | int  | The ID of the blog post to delete. |

#### Response

```txt
HTTP/1.1 200 OK
```


## Code Structure

The `src` directory contains the following Rust modules:

- `handler`: Contains the HTTP request handlers for the API endpoints.
- `model`: Contains the data model for posts.
- `repository`: Contains the SQLite database repository for storing and retrieving posts.
- `main.rs`: The main entry point for the application.

## License

This project is licensed under the terms of the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.
