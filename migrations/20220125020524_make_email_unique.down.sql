-- Add down migration script here
ALTER TABLE public.users DROP CONSTRAINT email_unique;