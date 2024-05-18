create table User (
    id unsigned int auto increment primary key not null,
    email varchar(120) not null unique,
    username varchar(120) unique,
    displayname varchar(120),
    pw_hash char(88) not null -- SHA512 in base64
);

create trigger User_no_email_username -- no username that's a different user's email
before insert on User
begin
    select raise(fail, "username is a different user's email")
    from User
    where email = NEW.username;
end;

create table Article (
    id char(32) primary key,
    -- UUID simple
    title varchar(200) not null,
    body text default ""
);
create table Author (
    user unsigned int not null,
    article char(32) not null,
    -- UUID simple
    unique (user, article)
);
create table Layouter (
    user unsigned int not null,
    article char(32) not null,
    unique (user, article)
);
create table Image (
    id unsigned int auto increment primary key,
    format varchar(15) not null,
    extension varchar(5) default "",
    caption varchar(200),
    source varchar(500)
);
create table ArticleImage (
    article unsigned int not null,
    image unsigned int not null,
    unique (article, image)
);
create table ArticleGroup (
    id unsigned int auto increment primary key,
    name varchar(200) not null,
    description text default "",
    locked boolean default false
);
create table ArticleMembership (
    article unsigned int not null,
    article_group unsigned int not null
);