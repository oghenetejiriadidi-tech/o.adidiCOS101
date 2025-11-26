use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_no: String,
    department: String,
    level: u32,
}

fn main() -> std::io::Result<()> {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_no: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_no: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_no: "CSC10110101".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_no: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_no: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    for student in &students {
        println!(
            "Name: {}, Matric: {}, Department: {}, Level: {}",
            student.name, student.matric_no, student.department, student.level
        );
    }

    // Write details into file
    let mut file = File::create("students_info.txt")?;
    for student in &students {
        writeln!(
            file,
            "Name: {}\nMatric No: {}\nDepartment: {}\nLevel: {}\n",
            student.name, student.matric_no, student.department, student.level
        )?;
    }

    println!("Student details saved to students_info.txt");
    Ok(())
}
