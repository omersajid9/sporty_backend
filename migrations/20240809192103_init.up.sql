-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS cube;
CREATE EXTENSION IF NOT EXISTS earthdistance;

create table
if not exists player (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username text unique not null,
    first_name varchar(100),
    last_name varchar(100),
    password text, -- yes, no, no
    auth_type varchar(10) not null, -- email, phone, apple
    auth_id text not null, -- yes, yes, yes
    profile_picture text not null default 'https://mact-profile-avatar.s3.us-east-1.amazonaws.com/images/id/AV3.png',
    UNIQUE (auth_type, auth_id)
);

insert into
player (username, password, auth_type, auth_id, profile_picture)
values ('omersajid', 'a', 'username', 'omersajid', 'https://mact-profile-avatar.s3.us-east-1.amazonaws.com/images/id/AV22.png');

insert into
player (username, password, auth_type, auth_id, profile_picture)
values ('omer', 'a', 'username', 'omer', 'https://mact-profile-avatar.s3.us-east-1.amazonaws.com/images/id/AV23.png');
insert into
player (username, password, auth_type, auth_id, profile_picture)
values ('omers', 'a', 'username', 'omers', 'https://mact-profile-avatar.s3.us-east-1.amazonaws.com/images/id/AV10.png');

insert into
player (username, password, auth_type, auth_id, profile_picture)
values ('omersa', 'a', 'username', 'omersa', 'https://mact-profile-avatar.s3.us-east-1.amazonaws.com/images/id/AV5.png');


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

-- insert into session (session_name, sport_id, host_id, location_name, lat, lon, public, max_players, start_time, end_time)
-- select 
--     'Session ' || i,
--     (select id from sport order by id offset (i - 1) % (select count(*) from sport) limit 1),  -- Cycle through sports
--     (select id from player order by id offset (i - 1) % (select count(*) from player) limit 1),  -- Cycle through players
--     'Location ' || i,
--     40.731680 + ((random() - 0.5) * 0.01),  -- Latitude with a small random offset
--     -74.055510 + ((random() - 0.5) * 0.01), -- Longitude with a small random offset
--     True,
--     2 + (i % 10),  -- Max players
--     NOW() + (CASE WHEN i % 2 = 0 THEN 10 * i ELSE 10 * i END || ' hours')::interval, -- Start time: alternating +/- i hours
--     NOW() + (CASE WHEN i % 2 = 0 THEN 10 * i + 2 ELSE (10 * i + 2) END || ' hours')::interval -- End time: start time + 2 hours
-- from generate_series(1, 4) as i;


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

-- INSERT INTO session_rsvp (session_id, player_id, player_rsvp, host_rsvp)
-- SELECT 
--     s.id AS session_id, 
--     p.id AS player_id,
--     CASE 
--         WHEN random() < 0.3 THEN 'Pending'
--         WHEN random() < 0.6 THEN 'Yes'
--         ELSE 'No'
--     END AS player_rsvp,
--     CASE 
--         WHEN random() < 0.3 THEN 'Pending'
--         WHEN random() < 0.6 THEN 'Yes'
--         ELSE 'No'
--     END AS host_rsvp
-- FROM 
--     session s
--     CROSS JOIN player p
-- WHERE 
--     p.id != s.host_id  -- Exclude the host from player RSVPs
-- LIMIT 50;  -- Limit

-- INSERT INTO team (name) VALUES 
-- ('Lightning Strikers'),
-- ('Mountain Wolves'),
-- ('Ocean Sharks'),
-- ('Urban Eagles'),
-- ('River Raiders');

-- INSERT INTO team_member (team_id, player_id)
-- SELECT 
--     t.id, 
--     p.id
-- FROM 
--     team t
-- CROSS JOIN 
--     player p
-- LIMIT 10;


-- INSERT INTO game (session_id, reporter_id, team_id_1, team_id_2, status)
-- SELECT 
--     s.id, 
--     p.id, 
--     t1.id, 
--     t2.id, 
--     (ARRAY['Pending', 'Yes', 'No'])[floor(random() * 3 + 1)::int]
-- FROM 
--     session s
-- JOIN 
--     player p ON p.id = s.host_id
-- CROSS JOIN 
--     team t1
-- CROSS JOIN 
--     team t2
-- WHERE 
--     t1.id != t2.id
-- LIMIT 10;


-- INSERT INTO score (game_id, team_id, score, round)
-- SELECT 
--     g.id, 
--     t.id, 
--     floor(random() * 10)::int, 
--     floor(random() * 3 + 1)::int
-- FROM 
--     game g
-- CROSS JOIN 
--     team t
-- WHERE 
--     t.id IN (g.team_id_1, g.team_id_2)
-- LIMIT 20;



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

create table
if not exists follow (
    user_id UUID not null,
    follower_id UUID not null,
    created_at timestamp not null default current_timestamp,
    primary key (follower_id, user_id),
    constraint fk_follower foreign key (follower_id) references player(id) on delete cascade,
    constraint fk_user foreign key (user_id) references player(id) on delete cascade
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