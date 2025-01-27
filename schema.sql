CREATE DATABASE "source-board"
    WITH
    OWNER = "source-board_owner"
    ENCODING = 'UTF8'
    LC_COLLATE = 'C'
    LC_CTYPE = 'C'
    LOCALE_PROVIDER = 'libc'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;

--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "osb_user" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"github_id" text NOT NULL,
	"username" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "osb_user_custom_config" (
	"id" text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"user_id" text NOT NULL,
	"repo_name" text NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
CREATE TABLE IF NOT EXISTS public.osb_session
(
    id text COLLATE pg_catalog."default" NOT NULL,
    user_id text COLLATE pg_catalog."default" NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT osb_session_pkey PRIMARY KEY (id),
    CONSTRAINT osb_session_user_id_osb_user_id_fk FOREIGN KEY (user_id)
        REFERENCES public.osb_user (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.osb_session
    OWNER to "source-board_owner";

-- DROP TABLE IF EXISTS public.osb_token;

CREATE TABLE IF NOT EXISTS public.osb_token
(
    id text COLLATE pg_catalog."default" NOT NULL DEFAULT gen_random_uuid(),
    user_id text COLLATE pg_catalog."default" NOT NULL,
    access_token text COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT osb_token_pkey PRIMARY KEY (id),
    CONSTRAINT osb_token_user_id_osb_user_id_fk FOREIGN KEY (user_id)
        REFERENCES public.osb_user (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.osb_token
    OWNER to "source-board_owner";

CREATE TABLE IF NOT EXISTS osb_github_logfiles (
    id text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    org_repo TEXT NOT NULL,
    logfile JSONB NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL

);