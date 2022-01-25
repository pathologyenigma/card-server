-- Add up migration script here
ALTER TABLE public.users
    ADD CONSTRAINT email_unique UNIQUE (email);