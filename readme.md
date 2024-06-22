# Shorts

## Installation

A simple backend application for creating and managing news shorts.

# Features

- User Authentication
- Create, Get Shorts

# Technologies Used

- Rust
- Axum (Web Framework)
- Postgres (Database)
- Diesel (ORM)

# Endpoints

- POST /user/signup
Create a new user

### Request Body
```json
{
    "name":"name",
    "email": "user@email.com",
    "password": "password"
}
```

- POST /user/login
Login a user
### Request Body
```json
{
    "email": "user@email.com",
    "password": "password"
}
```

- GET /shorts
Get all shorts

- POST /shorts/create-short
Create a new short
### Request Body
```json
{
    "title": "title",
    "descripton": "content",
    "ref_url":"https://www.google.com"
}
```