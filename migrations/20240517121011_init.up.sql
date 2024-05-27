create table User
(
    id          integer primary key autoincrement not null,
    email       varchar(120)                      not null unique,
    username    varchar(120) unique,
    display_name varchar(120),
    pw_hash     char(88)                          not null -- SHA512 in base64
);
create trigger User_no_duplicate_identifier -- no username that's a different user's email
    before
        insert
    on User
begin
    select raise(
                   fail,
                   'username is a different user''s email'
           )
    from User
    where NEW.username = email;

    select raise(fail, 'email is a different user''s username')
    from User
    where NEW.email = username;

    select raise(fail, 'username is a number')
    where cast(NEW.username as integer) is NEW.username;
end;

create table Article
(
    id    char(32) primary key not null,
    -- UUID simple
    title varchar(200)         not null,
    body  text default ''
);
create table Author
(
    user    integer  not null,
    article char(32) not null,
    -- UUID simple
    unique (user, article),
    foreign key (user) references User (id),
    foreign key (article) references Article (id)
);
create table Layouter
(
    user    integer  not null,
    article char(32) not null,
    unique (user, article),
    foreign key (user) references User (id),
    foreign key (article) references Article (id)
);
create table Image
(
    id        integer primary key autoincrement not null,
    format    varchar(15)                       not null,
    extension varchar(5) default null,
    caption   varchar(200),
    source    varchar(500)
);
create table ArticleImage
(
    article integer not null,
    image   integer not null,
    unique (article, image),
    foreign key (article) references Article (id),
    foreign key (image) references Image (id)
);
create table ArticleGroup
(
    id          integer primary key autoincrement not null,
    name        varchar(200)                      not null,
    description text    default '',
    locked      boolean default false
);
create table ArticleMembership
(
    article       integer not null,
    article_group integer not null,
    unique (article, article_group),
    foreign key (article) references Article (id),
    foreign key (article_group) references Article (id)

);
create table ArticleGroupMember
(
    article_group integer not null,
    user integer not null,
    status integer default 0, -- 0: invited; 1: accepted
    unique (article_group, user),
    foreign key (article_group) references ArticleGroup (id),
    foreign key (user) references User (id)
)