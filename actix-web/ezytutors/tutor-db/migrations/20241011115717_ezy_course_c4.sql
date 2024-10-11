-- Add migration script here
drop table if exists ezy_course_c4;
create table ezy_course_c4(
      course_id serial primary key,
      tutor_id INT not null,
      course_name varchar(140) not null,
      posted_time TIMESTAMP default now()
);

-- 테스트를 위한 시드 데이터
insert into ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
values (1, 1, 'First Course', '2024-10-10 20:11:00');
insert into ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
values (2, 1, 'Second Course', '2024-10-11 14:30:00');