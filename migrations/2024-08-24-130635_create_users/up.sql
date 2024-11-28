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
