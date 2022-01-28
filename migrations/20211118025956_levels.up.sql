CREATE TABLE public.level_settings
(
    id uuid NOT NULL,
    user_id integer NOT NULL,
    title character varying(120) COLLATE pg_catalog."default" NOT NULL,
    is_numberic_level boolean NOT NULL,
    levels jsonb NOT NULL,
    counts integer,
    tip_for_setting_user text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT level_settings_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.level_settings
    OWNER to postgres;

COMMENT ON COLUMN public.level_settings.title
    IS 'the title of the level setting, like a simple description of your level setting';

COMMENT ON COLUMN public.level_settings.is_numberic_level
    IS 'true for level like one star, two stars, three stars...
false for level like n, r, sr, ssr';

COMMENT ON COLUMN public.level_settings.levels
    IS 'numberic level will only have one value here.
non-numberic level will put all level names here.';

COMMENT ON COLUMN public.level_settings.counts
    IS 'only needs for numberic level';