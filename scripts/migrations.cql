CREATE KEYSPACE IF NOT EXISTS registry 
    WITH REPLICATION = {
        'class': 'SimpleStrategy', 
        'replication_factor': 1
    };

USE "registry";

CREATE TABLE IF NOT EXISTS config_entries (
	"id" bigint,
    "kind" int,
    "name" text,
    "meta" map<text, text>,
    "created_at" timestamp,
    "updated_at" timestamp,
    PRIMARY KEY (kind, id)
) WITH CLUSTERING ORDER BY (id DESC);