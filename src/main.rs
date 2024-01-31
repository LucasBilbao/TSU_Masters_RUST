use std::fs;
use std::io::stdin;

#[allow(dead_code)]
struct Subject {
    subject_name: String,
    grade: u16,
}
struct Student {
    first_name: String,
    last_name: String,
    subjects: Vec<Subject>,
}

impl Student {
    fn new(first_name: String, last_name: String, subjects: Vec<Subject>) -> Student {
        Student {
            last_name,
            first_name,
            subjects,
        }
    }

    fn students_that_passed_n_subjects(students: &mut Vec<Student>, n: usize) -> Vec<&mut Student> {
        students
            .into_iter()
            .filter(|student| {
                student
                    .subjects
                    .iter()
                    .filter(
                        |Subject {
                             subject_name: _,
                             grade,
                         }| *grade >= 51,
                    )
                    .collect::<Vec<&Subject>>()
                    .len()
                    == n
            })
            .collect::<Vec<&mut Student>>()
    }

    fn from_string(str: &str, separator: &str) -> Student {
        let info: Vec<&str> = str.split(separator).collect();

        let last_name = info[0].to_string();
        let first_name = info[1].to_string();

        let subjects: Vec<Subject> = info[2..]
            .iter()
            .map(|subject| {
                let subject_info: Vec<&str> = subject.split(':').collect();
                let subject_name = subject_info[0].to_string();
                let grade = subject_info[1].parse().unwrap_or(0) as u16;
                Subject {
                    subject_name,
                    grade,
                }
            })
            .collect();

        Student::new(first_name, last_name, subjects)
    }
}

fn main() {
    let file_content =
        fs::read_to_string("rust_data.txt").expect("Run into trouble while reading file");

    let students: &mut Vec<Student> = &mut vec![];

    for line in file_content.lines() {
        students.push(Student::from_string(line, "#"))
    }

    let passed_students;
    let mut n = String::new();
    println!("Please enter the number of subject that the student needs to have passed: ");
    stdin().read_line(&mut n).expect("Didn't Receive Input");
    let num_of_subjects = match n.trim().parse() {
        Ok(k) => {
            passed_students = Student::students_that_passed_n_subjects(students, k);
            k
        }
        Err(_) => panic!("\n\n\tThere was an error parsing the number you entered for n !!!\n\n"),
    };

    println!(
        "\tThe students that have passed {} subjects\n",
        num_of_subjects
    );

    for i in 0..passed_students.len() {
        let gpa: f32 = passed_students[i]
            .subjects
            .iter()
            .map(
                |Subject {
                     subject_name: _,
                     grade,
                 }| *grade,
            )
            .sum::<u16>() as f32
            / num_of_subjects as f32
            * 0.25;
        println!(
            "\t\t{}) {} {}, GPA: {:.2}",
            i + 1,
            passed_students[i].last_name,
            passed_students[i].first_name,
            gpa
        );
    }
}
