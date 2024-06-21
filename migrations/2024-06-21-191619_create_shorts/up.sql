-- Your SQL goes here
create table shorts(
    id text primary key,
    ref_url text not null,
    title text not null unique,
    description text not null,
    author text not null,
    foreign key (author) references users(email) on delete cascade
);