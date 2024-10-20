-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS cube;
CREATE EXTENSION IF NOT EXISTS earthdistance;

create table
if not exists player (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username varchar(50) unique not null,
    password text not null,
    date_of_birth date not null,
    -- location
    profile_picture text not null
);

insert into
player (username, password, date_of_birth, profile_picture)
values ('omersajid', 'a', '2000-10-16', 'https://avatar.iran.liara.run/public/22');

insert into
player (username, password, date_of_birth, profile_picture)
values ('omer', 'a', '2000-10-16', 'https://avatar.iran.liara.run/public/22');


create table
if not exists sport (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name varchar(100) unique not null,
    key varchar(100) unique not null,
    icon varchar(100) not null,
    icon_source varchar(100) not null
);

insert into
sport (name, key, icon, icon_source)
values ('Table Tennis', 'tabletennis', 'table-tennis', 'fontawesome5');

insert into
sport (name, key, icon, icon_source)
values ('Tennis', 'tennis', 'sports-tennis', 'materialicons');

insert into
sport (name, key, icon, icon_source)
values ('Pickle Ball', 'pickleball', 'racquetball', 'materialcommunityicons');

create table
if not exists rating (
    player_id UUID not null,
    sport_id UUID not null,
    rating double precision not null default 1500.0,
    std double precision not null default 350.0,
    val double precision not null default 0.6,
    updated timestamp not null default current_timestamp,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade,
    constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
    unique (player_id, sport_id)
);


create table
if not exists session (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    session_name text not null,
    sport_id UUID not null,
    host_id UUID not null,
    location_name text not null,
    lat double precision not null,
    lon double precision not null,
    public boolean not null default true,
    max_players int not null default 2,
    time timestamp not null default current_timestamp,
    constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
    constraint fk_host foreign key (host_id) references player(id) on delete cascade
);

-- create type session_player_rsvp as enum ('Pending', 'Yes', 'No');

create table
if not exists session_rsvp (
    session_id UUID not null,
    player_id UUID not null,
    player_rsvp text not null default 'Pending',
    host_rsvp text not null default 'Pending',
    primary key (session_id, player_id),
    constraint fk_session foreign key (session_id) references session(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);

create table
if not exists game (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    session_id UUID not null,
    player_id_1 UUID not null,
    player_id_2 UUID not null,
    status text not null default 'Pending',
    created_at timestamp not null default current_timestamp,
    constraint fk_session foreign key (session_id) references session(id) on delete cascade,
    constraint fk_player_1 foreign key (player_id_1) references player(id) on delete cascade,
    constraint fk_player_2 foreign key (player_id_2) references player(id) on delete cascade
);

create table
if not exists score (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    game_id UUID not null,
    player_id UUID not null,
    score int not null,
    round int not null,
    created_at timestamp not null default current_timestamp,
    constraint fk_game foreign key (game_id) references game(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);

-- create type challenge_rsvp as enum ('Maybe', 'Yes', 'No');

-- create table
-- if not exists challenge_request (
--     id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--     game_id UUID NOT NULL,
--     challenger_id UUID NOT NULL, -- The player challenging to join the game
--     status VARCHAR(50) NOT NULL DEFAULT 'Pending', -- Status can be 'Pending', 'Accepted', 'Rejected'
--     request_time timestamp not null default current_timestamp,
--     response_time timestamp, -- Optional, when host responds
--     constraint fk_game foreign key (game_id) references game(id) on delete cascade,
--     constraint fk_challenger foreign key (challenger_id) references player(id) on delete cascade
-- );


-- create type
-- friendship_status as enum ('Pending', 'accepted', 'declined', 'blocked');

-- create table 
-- if not exists friendship (
--     player_id_1 int not null,
--     player_id_2 int not null,
--     "status" friendship_status not null default 'Pending',
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