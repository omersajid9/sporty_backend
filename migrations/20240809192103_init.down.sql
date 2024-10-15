-- Add down migration script here

-- drop type session_player_rsvp cascade;
drop table if exists score;
drop table if exists session_rsvp;
drop table if exists game;
drop table if exists session;
drop table if exists rating;
drop table if exists player_sport;
drop table if exists sport;
drop table if exists player;
-- drop type player_game_rsvp;