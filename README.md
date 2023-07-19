# Tasks Manager App

### Rust Full Stack App covering Auth flow & CRUD with Rust/Axum and Supabase and Frontend with Yew.rs

In addition to having Rust installed on your machine, you must install the bundler for Yew.rs, called Trunk, and the target for WebAssembly:

```bash
$ rustup target add wasm32-unknown-unknown
```

and:

```bash
$ cargo install trunk
```

In the environment variables file, you must also set the settings you got when creating your project in Supabase:

```bash
// .env file
# Database settings
SUPABASE_URL=xxxx
SUPABASE_ANON_KEY=xxxx

# Jsonwebtoken settings
JWT_SECRET=my_ultra_secure_secret
JWT_EXPIRED_IN=60m
JWT_MAXAGE=60
```

In your project database in Supabase, you have to create 2 tables (users and tasks). For this purpose, you can use the SQL editor and paste the content of the "migrations" file:

```bash
// migrations file
create table
  public.users (
    id uuid not null default gen_random_uuid (),
    created_at timestamp with time zone not null default now(),
    username text not null,
    email text not null,
    password text not null,
    constraint user_pkey primary key (id),
    constraint user_email_key unique (email)
  ) tablespace pg_default;

create table
  public.tasks (
    id uuid not null default gen_random_uuid (),
    created_at timestamp with time zone not null default now(),
    title text not null,
    completed boolean null default false,
    description text not null,
    user_id uuid not null,
    constraint task_pkey primary key (id),
    constraint tasks_user_id_fkey foreign key (user_id) references users (id)
  ) tablespace pg_default;
```

Once these requirements are met, to run the application, you must first compile the code, both the backend and the frontend:

```bash
$ cd client/ && trunk build --release && cd.. && cargo run
```

The application will be available at http://localhost:8080.

## Deployment

If you deploy the application in production, in addition to passing the Supabase credentials as environment variables to the service you use to deploy it, you must previously compile the frontend by passing the deployment base address as environment variable:

```bash
API_ROOT=https://your-address.com trunk build --release
```
