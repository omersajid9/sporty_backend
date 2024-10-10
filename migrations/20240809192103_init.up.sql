-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS postgis;

create table
if not exists player (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username varchar(50) unique not null,
    password text not null,
    date_of_birth date not null,
    -- location
    profile_picture text
);

create table
if not exists sport (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name varchar(100) unique not null
);

insert into
sport (name)
values ('Table Tennis');

create table
if not exists player_sport (
    id UUID NOT NULL DEFAULT (uuid_generate_v4()),
    player_id UUID not null,
    sport_id UUID not null,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade,
    constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
    primary key (id),
    unique (player_id, sport_id)
);

create table
if not exists rating (
    player_sport_id UUID not null,
    rating int not null default 1500,
    std float not null default 350.0,
    val float not null default 0.5,
    constraint fk_player_sport foreign key (player_sport_id) references player_sport(id) on delete cascade,
    updated timestamp not null default current_timestamp
);


create table
if not exists game (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    sport_id UUID not null,
    host_id UUID not null,
    lat double precision not null,
    lon double precision not null,
    time timestamp not null default current_timestamp,
    constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
    constraint fk_host foreign key (host_id) references player(id) on delete cascade
);


-- CREATE TYPE player_game_rsvp AS ENUM ('Maybe', 'Yes', 'No');

create table
if not exists player_game (
    game_id UUID not null,
    player_id UUID not null,
    rsvp player_game_rsvp not null default 'Maybe',
    primary key (game_id, player_id),
    constraint fk_game foreign key (game_id) references game(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);



-- create type
-- friendship_status as enum ('pending', 'accepted', 'declined', 'blocked');

-- create table 
-- if not exists friendship (
--     player_id_1 int not null,
--     player_id_2 int not null,
--     "status" friendship_status not null default 'pending',
--     action_player_id UUID not null,
--     primary key (player_id_1, player_id_2),
--     constraint fk_player_1 foreign key (player_id_1) references player(id) on delete cascade,
--     constraint fk_player_2 foreign key (player_id_2) references player(id) on delete cascade,
--     constraint fk_action_player foreign key (action_player_id) references player(id) on delete cascade
-- );


-- insert into
-- sport (name)
-- values ('Table Tennis');


-- create table
-- if not exists game (
--     id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--     sport_id UUID not null,
--     host_id UUID not null,
--     location point not null,
--     time timestamp not null default current_timestamp,
--     constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
--     constraint fk_host foreign key (host_id) references player(id) on delete cascade
-- );

-- CREATE TYPE player_game_rsvp AS ENUM ('maybe', 'going', 'not_going');

-- create table
-- if not exists player_game (
--     game_id UUID not null,
--     player_id UUID not null,
--     rsvp player_game_rsvp not null default 'maybe',
--     primary key (game_id, player_id),
--     constraint fk_game foreign key (game_id) references game(id) on delete cascade,
--     constraint fk_player foreign key (player_id) references player(id) on delete cascade
-- );

-- create table
-- if not exists player_score (
--     id UUID NOT NULL DEFAULT (uuid_generate_v4()),
--     game_id UUID not null,
--     player_id UUID not null,
--     score point[] not null,
--     created_at timestamp not null default current_timestamp,
--     primary key (game_id, player_id),
--     constraint fk_game foreign key (game_id) references game(id) on delete cascade,
--     constraint fk_player foreign key (player_id) references player(id) on delete cascade
-- );