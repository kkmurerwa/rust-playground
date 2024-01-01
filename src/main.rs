
/// You are given a list of students with their scores as two lists. Create a student object to hold the student name
/// the student score and the respective grade of the student. Optionally add a comment based on the grade scored.
///
/// Grading system ==> A: 70-100, B: 60-69, C: 50-59, D: 40-49, F: 0-40
/// Comment System ==> A, B : Very Good, C : Average, D : Poor, E : Failure
fn main() {
    let names = ["James", "Sharon", "Edith", "Sean", "Alpha", "Angie", "Paul"];
    let scores: [i32; 7] = [100, 59, 61, 74, 80, 92, 39];

    // Create a vector to hold the student objects
    let mut students: Vec<Student> = Vec::new();

    // Iterate through the names and grades and create a student object
    for i in 0..7 {
        let score = scores[i];
        let grade = calculate_grade(&score);
        let comment = get_comment(&grade);
        let name = names[i];

        let student = Student {
            name,
            score,
            grade,
            comment
        };

        students.push(student);
    }

    for i in students {
        println!("Name: {}, Score: {}, Grade: {}, Comment: {}", i.name, i.score, i.grade, i.comment);
    }
}

fn calculate_grade(score: &i32) -> char {
    return match score {
        70..=100 => 'A',
        60..=69 => 'B',
        50..=59 => 'C',
        40..=49 => 'D',
        _ => 'F'
    };
}

fn get_comment(grade: &char) -> &'static str {
    let comment = match grade {
        'A' | 'B' => "Very Good",
        'C' => "Average",
        'D' => "Poor",
        _ => "Failure"
    };

    return comment;
}

// Student object
struct Student {
    name: &'static str,
    score: i32,
    grade: char,
    comment: &'static str,
}
