-- Add up migration script here
CREATE TABLE public.level_settings
(
    id uuid NOT NULL,
    user_id integer NOT NULL,
    name character varying(32) COLLATE pg_catalog."default" NOT NULL,
    datas jsonb NOT NULL,
    CONSTRAINT levels_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.level_settings
    OWNER to postgres;

COMMENT ON TABLE public.level_settings
    IS 'for every game or some thing that have card pool, everything in the card pool should have level, this table just for user to custom their owns, you can custom your level design.';

COMMENT ON COLUMN public.level_settings.user_id
    IS 'stands for one to many relation for users and this table';

COMMENT ON COLUMN public.level_settings.name
    IS 'the name of your level design';

COMMENT ON COLUMN public.level_settings.datas
    IS 'this is the settings and the items of your level, for example, here is a genshin impact style card pool level json:
{
  "name": "star",
  "numberic": "true",
  "color_changed": "true",
  "range": [ 3, 5 ],
  "colors": [
    {
      "r": 0,
      "g": 0,
      "b": 100
    },
    {
      "r": 160,
      "g": 32,
      "b": 240
    },
    {
      "r": 228,
      "g": 175,
      "b": 55
    }
  ],
  "items": {
    "names": "default",
    "images": "url here, if not numberic may be array"
  }
}';