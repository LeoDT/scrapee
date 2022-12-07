create table site
  (
    id integer primary key autoincrement not null,
    name text unique not null,
    save_context boolean not null default false,
    update_interval integer not null default 0,
    
    created_at text not null,
    updated_at text not null
  );

create table page
  (
    id integer primary key autoincrement not null,
    name text not null,
    url text,
    url_pattern text not null,
    
    site_id integer not null,
    
    created_at text not null,
    updated_at text not null,
    
    foreign key (site_id) references site (id)
  );

create table field
  (
    id integer primary key autoincrement not null,
    name text not null,
    xpath text not null,
    try_follow boolean not null default false,
    group_to text,
    
    page_id integer not null,
    
    created_at text not null,
    updated_at text not null,
    
    foreign key (page_id) references page (id)
  );

create table page_content
  (
    id integer primary key autoincrement not null,
    content text not null,
    url text not null,
    
    page_id integer not null,
    
    created_at text not null,
    updated_at text not null,
    
    foreign key (page_id) references page (id)
  );

CREATE UNIQUE INDEX page_content_url_index ON page_content (page_id, url);

create table page_content_archive
  (
    id integer primary key autoincrement not null,
    content text not null,
    url text not null,
    
    page_id integer not null,
    
    created_at text not null,
    
    foreign key (page_id) references page (id)
  );

create table job
  (
    id integer primary key autoincrement not null,
    kind integer not null,
    message text not null,
    status integer not null,

    fail_message text,
    fail_attempts integer not null default 0,

    created_at text not null,
    updated_at text not null,

    started_at text,
    successed_at text
  );
