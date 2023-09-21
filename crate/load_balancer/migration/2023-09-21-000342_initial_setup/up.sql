CREATE TABLE public.country (
	id serial4 NOT NULL,
	"name" varchar(100) NOT NULL,
	iso3 bpchar(3) NULL,
	numeric_code bpchar(3) NULL,
	iso2 bpchar(2) NULL,
	phonecode varchar(255) NULL,
	capital varchar(255) NULL,
	currency varchar(255) NULL,
	currency_name varchar(255) NULL,
	currency_symbol varchar(255) NULL,
	tld varchar(255) NULL,
	native varchar(255) NULL,
	region varchar(255) NULL,
	region_id int4 NULL,
	subregion varchar(255) NULL,
	subregion_id int4 NULL,
	nationality varchar(255) NULL,
	timezones text NULL,
	translations text NULL,
	latitude numeric(10, 8) NULL,
	longitude numeric(11, 8) NULL,
	emoji varchar(191) NULL,
	emojiu varchar(191) NULL,
	created_at timestamp NULL,
	updated_at timestamp NULL DEFAULT CURRENT_TIMESTAMP,
	wikidataid varchar(255) NULL,
	CONSTRAINT country_pkey PRIMARY KEY (id)
);

CREATE TABLE public."language" (
  id int4 NOT NULL,
  "name" text NOT NULL,
  CONSTRAINT language_name_key UNIQUE (name),
  CONSTRAINT language_pkey PRIMARY KEY (id)
);

CREATE TABLE public."user" (
	id uuid NOT NULL,
	"name" text NOT NULL,
	password_hash text NOT NULL,
	accout_name text NOT NULL,
	language_id int4 NOT NULL,
  CONSTRAINT user_language_id_key FOREIGN key (language_id) REFERENCES country(id),
	CONSTRAINT user_accout_name_key UNIQUE (accout_name),
	CONSTRAINT user_pkey PRIMARY KEY (id)
);

CREATE TABLE public."server" (
	id uuid NOT NULL,
	"name" text NOT NULL,
	country_id int4 NOT NULL,
	CONSTRAINT server_country_id_key FOREIGN key (country_id) REFERENCES country(id),
	CONSTRAINT server_name_key UNIQUE (name),
	CONSTRAINT server_pkey PRIMARY KEY (id)
);
