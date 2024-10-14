-- Add down migration script here

-- drop table if exists player_score;
drop type player_game_rsvp cascade;
drop table if exists player_game;
drop table if exists game;
drop table if exists rating;
drop table if exists player_sport;
drop table if exists sport;
-- drop table if exists friendship;
drop table if exists player;
-- drop type friendship_status;
-- drop type player_game_rsvp;