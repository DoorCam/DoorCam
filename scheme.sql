DROP TABLE flat;

CREATE TABLE flat (
   id INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   active BOOL NOT NULL,
   bell_button_pin INTEGER NOT NULL,
   local_address TEXT NOT NULL,
   broker_address TEXT NOT NULL,
   broker_port INTEGER NOT NULL,
   bell_topic TEXT NOT NULL,
   broker_user TEXT NOT NULL,
   broker_pw TEXT NOT NULL
   broker_pw_iv TEXT NOT NULL,
);

DROP TABLE client_user;

CREATE TABLE client_user (
   id INTEGER PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL UNIQUE,
   pw_hash TEXT NOT NULL,
   pw_salt TEXT NOT NULL,
   pw_config TEXT NOT NULL,
   user_type  INTEGER NOT NULL,
   active BOOL NOT NULL,
   flat_id INTEGER,
   FOREIGN KEY(flat_id) REFERENCES flat(id)
);

INSERT INTO client_user (name, pw_hash, pw_salt, pw_config, user_type, active) VALUES ("admin", "admin", "", "plain", 2, 1);
