CREATE TABLE public."jwt_token" (
  id uuid NOT NULL,
  "token" text NOT NULL,
  "active" bool NOT NULL,
  CONSTRAINT token_key UNIQUE (token),
  CONSTRAINT jwt_token_pkey PRIMARY KEY (id)
);
