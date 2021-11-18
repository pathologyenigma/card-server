-- Add down migration script here
DROP INDEX public.fki_users_id_fk_level_settings_user_id;
ALTER TABLE public.level_settings DROP CONSTRAINT users_id_fk_level_settings_user_id;