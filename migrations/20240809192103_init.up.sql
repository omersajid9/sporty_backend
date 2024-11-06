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
values ('omer', 'a', '2000-10-16', 'https://avatar.iran.liara.run/public/2');

insert into
player (username, password, date_of_birth, profile_picture)
values ('omers', 'a', '2000-10-16', 'https://avatar.iran.liara.run/public/3');

insert into
player (username, password, date_of_birth, profile_picture)
values ('omersa', 'a', '2000-10-16', 'https://avatar.iran.liara.run/public/40');


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
values ('Soccer', 'soccer', 'soccer-ball-o', 'fontawesome');

insert into
sport (name, key, icon, icon_source)
values ('Basketball', 'basketball', 'basketball', 'materialcommunityicons');

insert into
sport (name, key, icon, icon_source)
values ('Football', 'football', 'football', 'materialcommunityicons');

insert into
sport (name, key, icon, icon_source)
values ('Volleyball', 'volleyball', 'volleyball', 'fontawesome6');

insert into
sport (name, key, icon, icon_source)
values ('Cricket', 'cricket', 'cricket', 'materialcommunityicons');

insert into
sport (name, key, icon, icon_source)
values ('Baseball', 'baseball', 'baseball', 'fontawesome6');

insert into
sport (name, key, icon, icon_source)
values ('Running', 'running', 'person-running', 'fontawesome6');

insert into
sport (name, key, icon, icon_source)
values ('Cycling', 'cycling', 'bicycle', 'ionicons');

insert into
sport (name, key, icon, icon_source)
values ('Weight Lifting', 'weightlifting', 'weight-lifter', 'materialcommunityicons');


insert into
sport (name, key, icon, icon_source)
values ('Swimming', 'swimming', 'person-swimming', 'fontawesome6');

insert into
sport (name, key, icon, icon_source)
values ('Skateboarding', 'skateboarding', 'skateboarding', 'materialcommunityicons');

insert into
sport (name, key, icon, icon_source)
values ('Yoga', 'yoga', 'yoga', 'materialcommunityicons');

insert into
sport (name, key, icon, icon_source)
values ('Bowling', 'bowling', 'bowling-ball', 'fontawesome6');

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
    mode text not null, -- single or team
    rating double precision not null default 25.0,
    uncertainity double precision not null default 8.33,
    updated timestamp not null default current_timestamp,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade,
    constraint fk_sport foreign key (sport_id) references sport(id) on delete cascade,
    unique (player_id, sport_id, mode)
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
    start_time timestamp not null default current_timestamp,
    end_time timestamp not null default current_timestamp,
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
    unique(session_id, player_id),
    constraint fk_session foreign key (session_id) references session(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);

CREATE OR REPLACE FUNCTION notify_session_rsvp_insert()
RETURNS TRIGGER AS $$
BEGIN
    -- Notify the channel with the inserted row's ID as payload
    PERFORM pg_notify('session_rsvp_channel', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER session_rsvp_insert_trigger
AFTER INSERT ON session_rsvp
FOR EACH ROW
EXECUTE FUNCTION notify_session_rsvp_insert();

create table
if not exists team (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name text,
    created_at timestamp not null default current_timestamp
);

create table
if not exists team_member (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    team_id UUID not null,
    player_id UUID not null,
    created_at timestamp not null default current_timestamp,
    constraint fk_team foreign key (team_id) references team(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade,
    unique(team_id, player_id)
);

create table
if not exists game (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    session_id UUID not null,
    reporter_id UUID not null,
    team_id_1 UUID not null,
    team_id_2 UUID not null,
    status text not null default 'Pending',
    created_at timestamp not null default current_timestamp,
    constraint fk_session foreign key (session_id) references session(id) on delete cascade,
    constraint fk_reporter foreign key (reporter_id) references player(id) on delete cascade,
    constraint fk_team_1 foreign key (team_id_1) references team(id) on delete cascade,
    constraint fk_team_2 foreign key (team_id_2) references team(id) on delete cascade
);

CREATE OR REPLACE FUNCTION notify_game_insert()
RETURNS TRIGGER AS $$
BEGIN
    -- Notify the channel with the inserted row's ID as payload
    PERFORM pg_notify('game_channel', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER game_insert_trigger
AFTER INSERT ON game
FOR EACH ROW
EXECUTE FUNCTION notify_game_insert();

CREATE OR REPLACE FUNCTION notify_game_status_change()
RETURNS TRIGGER AS $$
BEGIN
    -- Only trigger notification if status is updated from 'Pending' to 'Yes' or 'No'
    IF OLD.status = 'Pending' AND (NEW.status = 'Yes' OR NEW.status = 'No') THEN
        PERFORM pg_notify('game_status_change_channel', row_to_json(NEW)::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger that calls the function after an update on the game table
CREATE TRIGGER game_status_update_trigger
AFTER UPDATE OF status ON game
FOR EACH ROW
EXECUTE FUNCTION notify_game_status_change();


create table
if not exists score (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    game_id UUID not null,
    team_id UUID not null,
    score int not null,
    round int not null,
    created_at timestamp not null default current_timestamp,
    constraint fk_game foreign key (game_id) references game(id) on delete cascade,
    constraint fk_team foreign key (team_id) references team(id) on delete cascade
);

create table
if not exists score_validation (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    game_id UUID not null,
    player_id UUID not null,
    status text not null,  -- 'approved' or 'rejected'  
    created_at timestamp not null default current_timestamp,
    constraint fk_game foreign key (game_id) references game(id) on delete cascade,
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);


-- CREATE OR REPLACE FUNCTION notify_score_insert()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     -- Notify the channel with the inserted row's ID as payload
--     PERFORM pg_notify('score_channel', row_to_json(NEW)::text);
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER score_insert_trigger
-- AFTER INSERT ON score_validation
-- FOR EACH ROW
-- EXECUTE FUNCTION notify_score_insert();

create table
if not exists notification_token (
    player_id UUID NOT NULL,
    token text not null,
    created_at timestamp not null default current_timestamp,
    CONSTRAINT unique_player_token UNIQUE (player_id, token),
    constraint fk_player foreign key (player_id) references player(id) on delete cascade
);

create table
if not exists notification (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    player_id UUID not null,
    channel text not null,
    message text not null,
    created_at timestamp not null default current_timestamp,
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