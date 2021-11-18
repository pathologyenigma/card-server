-- Add up migration script here
CREATE SEQUENCE public.cards_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 2147483647
    CACHE 1;

ALTER SEQUENCE public.cards_id_seq
    OWNER TO postgres;
CREATE TABLE public.cards
(
    id integer NOT NULL DEFAULT nextval('cards_id_seq'::regclass),
    name character varying(32) COLLATE pg_catalog."default" NOT NULL,
    level character varying(64) COLLATE pg_catalog."default" NOT NULL,
    level_setting_in_use uuid NOT NULL,
    extra_data jsonb NOT NULL,
    CONSTRAINT cards_pkey PRIMARY KEY (id),
    CONSTRAINT characters_level_setting_in_use_fk_level_settings_id FOREIGN KEY (level_setting_in_use)
        REFERENCES public.level_settings (id) MATCH SIMPLE
        ON UPDATE SET NULL
        ON DELETE CASCADE
);

ALTER TABLE public.cards
    OWNER to postgres;

COMMENT ON COLUMN public.cards.extra_data
    IS 'this just represent your card''s data, maybe it contains image, live2d, mesh or some other things, it depends on how client show out the card.
here is a example:
{
  "images": [
    {
      "type": "head_portrait/normal",
      "url": "https://xxxxxxx",
      "subimgs": null
    },
    {
      "type": "sprite",
      "url": "https://xxxxxxx",
      "subimg": {
        "width": 500,
        "height": 500,
        "count": 3,
        "children": [
          {
            "achor": [ 0, 0 ],
            "width": 200,
            "height": 200,
            "border": null
          },
          {
            "comment": "just like the last one"
          },
          {
            "comment": "this array'' len should be same as the count"
          }
        ]
      }
    },
    {
      "type": "animation2d",
      "url": null,
      "subimg": {
        "width": 500,
        "height": 500,
        "count": 3,
        "ticks": 20,
        "children": [
          {
            "url": "https://xxxxxxxx",
            "ticks": [ 0, 5 ],
            "fill_type": "cover"
          },
          {
            "url": "https://xxxxxxxx",
            "ticks": [ 5, 10 ],
            "fill_type": "auto"
          },
          {
            "url": "https://xxxxxxxx",
            "ticks": [ 10, 20 ],
            "fill_type": "fill"
          }
        ]
      }
    }
  ],
  "labels": [
    {
      "name": "name",
      "value": "character_1"
    },
    {
      "name": "description",
      "value": "it is a example character label data"
    }
  ],
  "numbers": [
    {
      "name": "hp",
      "type": "float",
      "value": 1.0
    }
  ],
  "comment": "this is needed only when you need info more than just card draw records"
}';