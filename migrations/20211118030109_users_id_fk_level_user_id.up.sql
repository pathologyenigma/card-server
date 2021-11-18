-- Add up migration script here
ALTER TABLE public.level_settings
    ADD CONSTRAINT users_id_fk_level_settings_user_id FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE SET NULL
    ON DELETE CASCADE;
CREATE INDEX fki_users_id_fk_level_settings_user_id
    ON public.level_settings USING btree
    (user_id ASC NULLS LAST)
    TABLESPACE pg_default;