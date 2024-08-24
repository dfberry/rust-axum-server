-- Your SQL goes here
CREATE TABLE watches (
    id text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    github_user_id text NOT NULL,
    org_repo_name text NOT NULL,
    watch_type VARCHAR(30) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

ALTER TABLE "watches" ADD CONSTRAINT "watches_user_githu_buser_users_id_fk" FOREIGN KEY ("github_user_id") REFERENCES "public"."users"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

CREATE UNIQUE INDEX IF NOT EXISTS "unique_user_watch" ON "watches" USING btree ("github_user_id", "org_repo_name", "watch_type");