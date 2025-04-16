# Plan: Full-Stack Todo App with Rust, Dioxus, and SQLite

This document outlines the steps to build a full-stack todo application using Rust for both the backend and frontend (via WebAssembly with Dioxus), and SQLite for the database.

## Phase 1: Backend Development (Rust + SQLite)

1.  **Project Setup:**
    *   Initialize a new Rust binary project for the backend server (e.g., `cargo new todo_backend --bin`).
    *   Choose a web framework: Axum (recommended for its integration with Tokio) or Actix-web. Add the chosen framework and `tokio` as dependencies.
    *   Choose a database interaction library: SQLx (recommended for compile-time query checking) or Diesel. Add `sqlx-sqlite` and `sqlx` (with necessary features like `runtime-tokio-rustls`, `macros`, `sqlite`) or `diesel` (with `sqlite` feature) as dependencies.
    *   Add `serde` for serialization/deserialization (`features = ["derive"]`).
    *   Add `dotenvy` for managing environment variables (like `DATABASE_URL`).

2.  **Database Setup:**
    *   Define the database schema for the `todos` table (e.g., `id` INTEGER PRIMARY KEY, `task` TEXT NOT NULL, `completed` BOOLEAN NOT NULL DEFAULT 0).
    *   Use SQLx CLI (`sqlx migrate add create_todos_table`) or Diesel CLI (`diesel setup`, `diesel migration generate create_todos`) to create migration files.
    *   Write the SQL `CREATE TABLE` statement in the migration file.
    *   Configure the `DATABASE_URL` environment variable (e.g., `DATABASE_URL=sqlite:todo.db`).
    *   Run the migrations (`sqlx migrate run` or `diesel migration run`).

3.  **API Design & Implementation:**
    *   Define the `Todo` struct (with `id`, `task`, `completed`) and derive `serde::Serialize`, `serde::Deserialize`, and potentially `sqlx::FromRow`.
    *   Define API endpoints for CRUD operations:
        *   `GET /api/todos`: List all todos.
        *   `POST /api/todos`: Create a new todo (request body: `{ "task": "..." }`).
        *   `GET /api/todos/:id`: Get a specific todo (optional).
        *   `PUT /api/todos/:id`: Update a todo (e.g., mark as complete/incomplete) (request body: `{ "task": "...", "completed": ... }`).
        *   `DELETE /api/todos/:id`: Delete a todo.
    *   Implement handler functions for each endpoint using the chosen web framework and database library.
        *   Establish a database connection pool (e.g., `SqlitePool` for SQLx) and share it with handlers (e.g., using Axum state or Actix-web app data).
        *   Write SQL queries (using SQLx macros or Diesel query builder) to interact with the database.
        *   Handle request parsing, database operations, and response serialization (JSON).
    *   Implement basic error handling.

4.  **Testing (Optional but Recommended):**
    *   Write integration tests for the API endpoints.

## Phase 2: Frontend Development (Dioxus + WebAssembly)

1.  **Project Setup:**
    *   Use `dx` (Dioxus CLI) or manually set up a new Rust library project for the frontend (`cargo new todo_frontend --lib`).
    *   Add `dioxus` (with `web` feature) as a dependency.
    *   Add `dioxus-router` for routing (if needed, though likely simple for a todo app).
    *   Add `reqwasm` or similar for making HTTP requests from WASM.
    *   Add `serde` for serialization/deserialization.
    *   Configure `Cargo.toml` for WASM compilation (`[lib] crate-type = ["cdylib", "rlib"]`).
    *   Create an `index.html` file to load the WASM module.
    *   Install `wasm-pack` and potentially `dx` CLI (`cargo install dioxus-cli`).

2.  **Component Structure:**
    *   Define the main `App` component.
    *   Create components for:
        *   `TodoList`: Displays the list of todos.
        *   `TodoItem`: Represents a single todo item (displaying task, checkbox for completion, delete button).
        *   `AddTodoForm`: Input field and button to add new todos.

3.  **State Management:**
    *   Use Dioxus `Signal`s or `use_state` hooks to manage the list of todos fetched from the backend.
    *   Manage the state of the input field in `AddTodoForm`.

4.  **API Integration:**
    *   Define the `Todo` struct (matching the backend) and derive `serde::Serialize`, `serde::Deserialize`, `Clone`, `PartialEq`.
    *   Implement functions to interact with the backend API using `reqwasm`:
        *   `fetch_todos()`: Calls `GET /api/todos`.
        *   `add_todo(task: String)`: Calls `POST /api/todos`.
        *   `update_todo(id: i32, completed: bool)`: Calls `PUT /api/todos/:id`.
        *   `delete_todo(id: i32)`: Calls `DELETE /api/todos/:id`.
    *   Use Dioxus `use_resource` or `use_future` hooks to fetch initial todos when the `App` component mounts.
    *   Update the local state after successful API calls (add, update, delete).

5.  **UI Implementation:**
    *   Implement the rendering logic for each component using Dioxus RSX syntax.
    *   Handle user interactions (typing in the input, clicking checkboxes, clicking delete buttons) and trigger corresponding API calls and state updates.

6.  **Styling:**
    *   Add basic CSS styling for the application in a separate CSS file linked in `index.html`.

## Phase 3: Integration & Build

1.  **CORS Configuration:**
    *   Configure Cross-Origin Resource Sharing (CORS) on the backend server to allow requests from the frontend's origin (usually `localhost` during development). Use crates like `tower-http` (for Axum) or `actix-cors` (for Actix-web).

2.  **Build Process:**
    *   Build the frontend WASM module: `wasm-pack build --target web ./todo_frontend`.
    *   Build the backend executable: `cargo build --release --bin todo_backend`.
    *   Optionally, configure the backend server to serve the static frontend files (HTML, CSS, JS, WASM) from a specific directory (e.g., `dist` or `static`).

3.  **Running the Application:**
    *   Start the backend server.
    *   Serve the `index.html` and associated static files (either via the backend or a simple static file server like `miniserve`).
    *   Open the `index.html` in a web browser.

## Phase 4: Deployment (Optional)

1.  **Backend Deployment:**
    *   Choose a hosting provider (e.g., Fly.io, Shuttle, Render, VPS).
    *   Containerize the backend application using Docker (optional but recommended).
    *   Configure the production environment (database URL, CORS origins).
    *   Deploy the backend application.

2.  **Frontend Deployment:**
    *   Deploy the static frontend files (HTML, CSS, JS, WASM) to a static hosting provider (e.g., Netlify, Vercel, GitHub Pages) or serve them from the backend server.

3.  **Database:**
    *   Ensure the SQLite database file is persisted correctly in the deployment environment. Consider alternatives like PostgreSQL if scaling becomes a concern.

This plan provides a structured approach. Details within each step might vary based on specific library choices and implementation preferences.

</final_file_content>

IMPORTANT: For any future changes to this file, use the final_file_content shown above as your reference. This content reflects the current state of the file, including any auto-formatting (e.g., if you used single quotes but the formatter converted them to double quotes). Always base your SEARCH/REPLACE operations on this final version to ensure accuracy.
