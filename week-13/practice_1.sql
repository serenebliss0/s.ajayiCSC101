-- Active: 1761903832511@@127.0.0.1@5432@globacom_dbase
ALTER DATABASE mtnnigeria_db
RENAME TO globacom_db;

ALTER DATABASE globacom_db
RENAME TO globacom_db;

ALTER TABLE employees
RENAME TO staff;

ALTER TABLE staff
RENAME COLUMN phone_number TO mobile;

ALTER TABLE staff
RENAME COLUMN employee_id TO staff_id;

ALTER TABLE staff
RENAME COLUMN employee_name TO staff_name;
ALTER TABLE staff
RENAME COLUMN employee_salary TO staff_salary;

ALTER TABLE staff
ALTER COLUMN mobile TYPE VARCHAR(15);

ALTER TABLE staff
ADD COLUMN gender VARCHAR(1);

ALTER TABLE staff
ALTER COLUMN mobile DROP NOT NULL;

ALTER TABLE staff
ALTER COLUMN mobile SET NOT NULL;

ALTER TABLE staff
DROP COLUMN gender;

SELECT staff_id, staff_name, staff_salary FROM staff;

SELECT * FROM staff
WHERE staff_salary = 120000;

SELECT * FROM staff
WHERE age > 40;

SELECT * FROM staff
WHERE age > 40 AND staff_salary > 400000;

SELECT * FROM staff
WHERE age IS NOT NULL;

SELECT * FROM staff
WHERE staff_name LIKE 'AI%';

SELECT * FROM staff
WHERE age IN (30, 35, 40);

SELECT * FROM staff
WHERE age NOT IN (30, 35);

SELECT * FROM staff
WHERE age BETWEEN 33 AND 44;

SELECT * FROM staff;

UPDATE staff
SET mobile = '7030089981'
WHERE staff_id = '117';

SELECT s.staff_id, s.staff_name, d.department_number, d.department_name 
FROM staff s, departments d;

SELECT s.staff_id, s.staff_name, d.department_number, d.department_name
FROM staff s, departments d;

SELECT s.staff_id, s.staff_name, d.department_location, p.project_number, p.project_name
FROM departments d, staff s, project p
WHERE p.project_name = 'C' AND d.department_number = '1';

DELETE FROM staff
WHERE staff_name = 'Alade Joy';

CREATE TABLE customers (
    customer_id SERIAL PRIMARY KEY NOT NULL,
    customer_name TEXT NOT NULL,
    customer_age INT CHECK (customer_age > 0),
    customer_email TEXT NOT NULL,
    customer_mobile VARCHAR(15),
    employee_id INT NOT NULL,
    data_id INT NOT NULL
);

-- 1. Create a sequence
--CREATE SEQUENCE customers_customer_id_seq START 110;

-- 2. Attach it to the existing column


ALTER TABLE customers
ALTER COLUMN customer_id SET DEFAULT nextval('customers_customer_id_seq');
INSERT INTO customers (customer_id, customer_name, customer_age, customer_email, customer_mobile, employee_id, data_id) 
VALUES
(113, 'Philip Akonjo', 41, 'p_akonjo@gmail.com', '09052356772', 100, 2),
(114, 'Marylene Mapa', 33, 'm_mapa@gmail.com', '08053333551', 120, 5),
(115, 'Oghenero Agor', 50, 'o_agor@gmail.com', '07055566774', 117, 11),
(116, 'Adams Bree', 33, 'a_bree@gmail.com', '08056765424', 102, 1),
(;117, 'Okafor Mathias', 45, 'o_mathias@gmail.com', '08056763367', 120, 10),
(118, 'Samson Adeleke', 65, 's_adeleke@gmail.com', '07056774423', 117, 11),
(119, 'Lawal Tamire', 35, 'l_tamire@gmail.com', '09052111101', 107, 5),
(120, 'James Job', 44, 'j_job@gmail.com', '08059693919', 100, 8),
(121, 'Matthew Jakande', 21, 'm_jakande@gmail.com', '07051232144', 120, 2),
(122, 'Jimila Adegboye', 20, 'j_adegboye@gmail.com', '08054921923', 107, 5);

CREATE TABLE dataplan (
    data_id SERIAL PRIMARY KEY NOT NULL,
    data_size TEXT NOT NULL,
    data_duration INT NOT NULL,
    data_price INT NOT NULL
);

INSERT INTO dataplan (data_id, data_size, data_duration, data_price)
VALUES
(1,  '350MB',  2,  200),
(2,  '1.8GB', 14,  500),
(3,  '3.9GB', 30, 1000),
(4,  '7.5GB', 30, 1500),
(5,  '9.2GB', 30, 2000),
(6, '10.8GB', 30, 2500),
(7,   '14GB', 30, 3000),
(8,   '18GB', 30, 4000),
(9,   '24GB', 30, 5000),
(10, '29.9GB', 30, 8000),
(11,  '50GB', 30, 10000);

