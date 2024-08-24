-- This file should undo anything in `up.sql`
-- Drop the unique index
DROP INDEX IF EXISTS "unique_user_watch";

-- Drop the foreign key constraint
ALTER TABLE "watches" DROP CONSTRAINT IF EXISTS "watches_user_githubuser_users_id_fk";

DROP TABLE IF EXISTS "watches";
